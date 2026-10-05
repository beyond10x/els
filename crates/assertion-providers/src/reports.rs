use crate::contract::TestReport as Tests;
use crate::{Arguments, Collected, ProviderContext, ProviderError, arg, known};
use serde_json::{Value, json};
use std::collections::BTreeSet;
fn integer(v: &Value, key: &str) -> Result<u64, ProviderError> {
    v.get(key).and_then(Value::as_u64).ok_or_else(|| {
        ProviderError::Invalid(format!("report field {key} must be a nonnegative integer"))
    })
}
fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str, ProviderError> {
    v.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            ProviderError::Invalid(format!("report field {key} must be a nonempty string"))
        })
}
fn array<'a>(v: &'a Value, key: &str) -> Result<&'a Vec<Value>, ProviderError> {
    v.get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| ProviderError::Invalid(format!("report field {key} must be an array")))
}
fn sum(values: &[u64]) -> Result<u64, ProviderError> {
    values.iter().try_fold(0u64, |a, b| {
        a.checked_add(*b)
            .ok_or_else(|| ProviderError::Invalid("report count overflow".into()))
    })
}
pub(super) fn collect(
    operation: &str,
    args: &Arguments,
    context: &ProviderContext,
) -> Result<Collected, ProviderError> {
    let (report, exit_code) = if operation == "tests.run" {
        match crate::process::collect(args, context)? {
            Collected::Unavailable(reason) => return Ok(Collected::Unavailable(reason)),
            Collected::Known(output) => (
                serde_json::from_str(string(&output, "stdout")?)?,
                Some(output["exit_code"].as_i64().ok_or_else(|| {
                    ProviderError::Invalid("test process ended without an exit code".into())
                })?),
            ),
        }
    } else {
        (
            crate::files::document(context, arg(args, "path")?, "json")?,
            None,
        )
    };
    match operation {
        "tests.report" | "tests.run" => {
            let passed = integer(&report, "passed")?;
            let failed = integer(&report, "failed")?;
            let skipped = integer(&report, "skipped")?;
            let tests: Tests = serde_json::from_value(report)?;
            if tests.format != "engineering-tests/1" {
                return Err(ProviderError::Invalid(
                    "unsupported tests report format".into(),
                ));
            }
            let total = sum(&[passed, failed, skipped])?;
            let executed = sum(&[passed, failed])?;
            if exit_code.is_some_and(|code| (code == 0) != (failed == 0)) {
                return Err(ProviderError::Invalid(
                    "test process exit contradicts normalized report".into(),
                ));
            }
            let unique: BTreeSet<_> = tests.inventory.iter().collect();
            if executed == 0
                || total != tests.inventory.len() as u64
                || unique.len() != tests.inventory.len()
                || tests.inventory.iter().any(String::is_empty)
            {
                return Err(ProviderError::Invalid(
                    "test report requires unique complete inventory and nonzero execution".into(),
                ));
            }
            known(
                json!({"inventory":tests.inventory,"passed":tests.passed,"failed":tests.failed,"skipped":tests.skipped,"total":total,"executed":executed}),
            )
        }
        "codegate.dependencies" => {
            if string(&report, "format")? != "codegate-dependency-report/0.1" {
                return Err(ProviderError::Invalid(
                    "unsupported Codegate dependency report format".into(),
                ));
            }
            let source = string(&report, "source_id")?;
            let config = string(&report, "configuration_id")?;
            if string(&report, "metric_id")? != "dependency.unique-fan-out/1"
                || string(&report, "checker_id")? != "dependency.forbidden-edge/1"
            {
                return Err(ProviderError::Invalid(
                    "unsupported Codegate checker or metric".into(),
                ));
            }
            let verdict_wire = string(&report, "verdict")?;
            let coverage_wire = string(&report, "coverage")?;
            if !["Pass", "Fail", "Incomplete", "Unsupported", "Error"].contains(&verdict_wire)
                || !["Complete", "Partial", "Unsupported", "Failed"].contains(&coverage_wire)
            {
                return Err(ProviderError::Invalid(
                    "invalid Codegate verdict or coverage".into(),
                ));
            }
            let verdict = verdict_wire.to_ascii_lowercase();
            let coverage = coverage_wire.to_ascii_lowercase();
            let findings = array(&report, "findings")?;
            let fanout = array(&report, "fan_out")?;
            let _ = array(&report, "diagnostics")?;
            let policy = report
                .get("policy")
                .ok_or_else(|| ProviderError::Invalid("Codegate policy missing".into()))?;
            if (string(policy, "format")? != "codegate-dependency-policy/0.1"
                || string(policy, "expected_source_id")? != source
                || string(policy, "expected_configuration_id")? != config)
                && (verdict == "pass" || verdict == "fail")
            {
                return Err(ProviderError::Invalid(
                    "Codegate policy identity mismatch".into(),
                ));
            }
            if verdict == "pass" && (coverage != "complete" || !findings.is_empty()) {
                return Err(ProviderError::Invalid(
                    "inconsistent passing Codegate report".into(),
                ));
            }
            if verdict == "fail" && findings.is_empty() {
                return Err(ProviderError::Invalid(
                    "Codegate failure requires a finding".into(),
                ));
            }
            let mut maximum = 0;
            for entry in fanout {
                let _ = string(entry, "unit_id")?;
                if entry.get("value").is_some_and(|v| !v.is_null()) {
                    maximum = maximum.max(integer(entry, "value")?);
                } else if coverage == "complete" {
                    return Err(ProviderError::Invalid(
                        "complete Codegate coverage requires known fan-out values".into(),
                    ));
                }
            }
            for finding in findings {
                for field in ["edge_id", "source", "target", "kind"] {
                    let _ = string(finding, field)?;
                }
            }
            if verdict != "pass" && verdict != "fail" {
                return Ok(Collected::Unavailable(format!(
                    "Codegate report verdict is {verdict}"
                )));
            }
            known(
                json!({"verdict":verdict,"coverage":coverage,"findings_count":findings.len(),"fan_out_max":maximum,"source_id":source,"configuration_id":config}),
            )
        }
        "ess.report" => ess(&report),
        _ => Err(ProviderError::Invalid("unknown report operation".into())),
    }
}
fn ess(report: &Value) -> Result<Collected, ProviderError> {
    if string(report, "format")? != "ess-conformance-report/2" {
        return Err(ProviderError::Invalid(
            "unsupported ESS conformance report format".into(),
        ));
    }
    for field in [
        "specification",
        "spec_digest",
        "implementation",
        "producer_profile",
        "policy",
    ] {
        let _ = string(report, field)?;
    }
    if string(report, "policy")? != "complete-selection/1" {
        return Err(ProviderError::Invalid(
            "unsupported ESS qualification policy".into(),
        ));
    }
    let counts = report
        .get("counts")
        .ok_or_else(|| ProviderError::Invalid("ESS report missing counts".into()))?;
    let categories = ["passed", "failed", "unsupported", "error", "skipped"];
    let values = categories
        .map(|key| integer(counts, key))
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    let total = sum(&values)?;
    if total == 0 || integer(counts, "total")? != total {
        return Err(ProviderError::Invalid(
            "ESS report zero or inconsistent total".into(),
        ));
    }
    let outcomes = report
        .get("outcomes")
        .ok_or_else(|| ProviderError::Invalid("ESS missing outcomes".into()))?;
    let mut inventory = BTreeSet::new();
    for (category, count) in categories.iter().zip(&values) {
        let ids = array(outcomes, category)?;
        if ids.len() as u64 != *count {
            return Err(ProviderError::Invalid("ESS category count mismatch".into()));
        }
        let mut previous = None;
        for id in ids {
            let name = id
                .as_str()
                .filter(|s| !s.is_empty())
                .ok_or_else(|| ProviderError::Invalid("invalid ESS scenario id".into()))?;
            if previous.is_some_and(|p| p >= name) || !inventory.insert(name) {
                return Err(ProviderError::Invalid(
                    "ESS outcomes require globally distinct sorted IDs".into(),
                ));
            }
            previous = Some(name);
        }
    }
    let status = string(report, "conformance_status")?;
    let execution = string(report, "execution_status")?;
    if !["passed", "failed", "inconclusive"].contains(&status)
        || !["passed", "failed", "inconclusive"].contains(&execution)
    {
        return Err(ProviderError::Invalid("invalid ESS status".into()));
    }
    let suite = report
        .get("suite")
        .ok_or_else(|| ProviderError::Invalid("ESS suite missing".into()))?;
    for field in ["version", "digest_profile", "digest"] {
        let _ = string(suite, field)?;
    }
    let _ = integer(report, "completed_at")?;
    let coverage = report
        .get("coverage")
        .ok_or_else(|| ProviderError::Invalid("ESS coverage missing".into()))?;
    let knowledge = string(coverage, "knowledge")?;
    let complete = if knowledge == "complete_inventory" {
        let covered = coverage
            .get("counts")
            .ok_or_else(|| ProviderError::Invalid("ESS coverage counts missing".into()))?;
        let refused = array(coverage, "refused")?;
        if sum(&[
            integer(covered, "generated")?,
            integer(covered, "authored")?,
        ])? != total
            || integer(covered, "refused")? != refused.len() as u64
        {
            return Err(ProviderError::Invalid(
                "ESS coverage counts disagree".into(),
            ));
        }
        let _ = integer(covered, "outside")?;
        let selection = coverage
            .get("selection")
            .ok_or_else(|| ProviderError::Invalid("ESS selection missing".into()))?;
        if !selection.is_object() {
            return Err(ProviderError::Invalid(
                "ESS selection must be an object".into(),
            ));
        }
        let mut complete = true;
        for refusal in refused {
            if string(refusal, "scope")? == "in_scope" {
                complete = false;
            }
        }
        complete
    } else if knowledge == "unknown" {
        false
    } else {
        return Err(ProviderError::Invalid("unsupported ESS knowledge".into()));
    };
    let all_pass = values[0] == total;
    if execution == "passed" && !all_pass
        || status == "passed" && (!all_pass || !complete || execution != "passed")
    {
        return Err(ProviderError::Invalid(
            "inconsistent ESS passing report".into(),
        ));
    }
    known(
        json!({"passed":values[0],"failed":values[1],"unsupported":values[2],"errors":values[3],"skipped":values[4],"total":total,"inventory":inventory,"conformance_status":status,"spec_digest":string(report,"spec_digest")?,"implementation":string(report,"implementation")?}),
    )
}
