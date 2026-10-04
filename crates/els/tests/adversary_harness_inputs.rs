//! Adversary pass 1 on `story:fixture-harness`: inputs the harness reads differently from Canon,
//! and instants the `els-fixture/1` format says it refuses.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use support::Fixture;

const SMOKE: &str = "fixtures/smoke/smoke.fixture.yaml";

fn smoke_text() -> String {
    std::fs::read_to_string(support::repo_root().join(SMOKE)).expect("the smoke fixture reads")
}

fn smoke_with(from: &str, to: &str) -> String {
    let text = smoke_text();
    assert!(text.contains(from), "the smoke fixture contains `{from}`");
    text.replacen(from, to, 1)
}

/// The harness says it evaluates with Canon and adds no reader of its own, but it deserializes
/// the case and the evidence records straight from the fixture text. Canon's own readers
/// (`read_case`, `read_evidence`, the path `canon evaluate` takes) refuse an identifier, revision
/// or result written as a number or a boolean as `malformed-input`. The same record inside a
/// fixture must not load.
#[test]
fn adversary_harness_refuses_a_case_or_record_canon_refuses_to_read() {
    // Premise: Canon's readers refuse these exact documents.
    let case = "format: canon-case/1\nid: SMOKE-1\nprotocol: smoke\nartifacts:\n  service: {revision: 1}\n";
    let refusal = b10x_canon::eval::read_case(case).expect_err("Canon refuses `revision: 1`");
    assert_eq!(refusal.code(), "malformed-input", "{refusal}");
    let record = "format: canon-evidence/1\nid: observation-1\nkind: operational_observation\n\
                  result: healthy\nsubject: service\nsubject_revision: 1\n";
    let refusal =
        b10x_canon::eval::read_evidence(record).expect_err("Canon refuses `subject_revision: 1`");
    assert_eq!(refusal.code(), "malformed-input", "{refusal}");
    let record = "format: canon-evidence/1\nid: observation-1\nkind: operational_observation\n\
                  result: true\nsubject: service\nsubject_revision: r1\n";
    let refusal =
        b10x_canon::eval::read_evidence(record).expect_err("Canon refuses `result: true`");
    assert_eq!(refusal.code(), "malformed-input", "{refusal}");

    // The harness must refuse the same documents inside a fixture.
    let mut loaded = Vec::new();
    for (from, to) in [
        ("service: {revision: r1}", "service: {revision: 1}"),
        ("subject_revision: r1", "subject_revision: 1"),
        ("result: healthy", "result: true"),
    ] {
        if let Ok(fixture) = Fixture::from_yaml(&smoke_with(from, to)) {
            loaded.push(format!("`{to}` loads as {:?}", fixture.case()));
        }
    }
    assert!(
        loaded.is_empty(),
        "the harness reads what Canon refuses: {loaded:?}"
    );
}

/// `is_instant` documents "each field in range", and the format promises a real UTC instant. A
/// day that does not exist in its month is not one.
#[test]
fn adversary_harness_refuses_an_instant_that_is_not_a_calendar_date() {
    let mut loaded = Vec::new();
    for at in [
        "2026-02-29T00:00:00Z", // 2026 is not a leap year
        "2026-02-30T00:00:00Z",
        "2026-04-31T00:00:00Z",
    ] {
        let text = smoke_with("at: \"2026-10-04T00:00:00Z\"", &format!("at: \"{at}\"")).replace(
            "observed_at: \"2026-10-03T23:00:00Z\"",
            "observed_at: \"2026-01-01T00:00:00Z\"",
        );
        if Fixture::from_yaml(&text).is_ok() {
            loaded.push(at);
        }
    }
    assert!(
        loaded.is_empty(),
        "instants that name no day load: {loaded:?}"
    );
}

/// The boundaries of the one instant form. Every one of these is refused today; each kills a
/// mutant of `is_instant` the existing suite does not (hour `<= 24`, minute or second `<= 60`,
/// month or day ranges widened, the `Z` or `T` literal dropped).
#[test]
fn adversary_harness_instant_boundaries_are_refused() {
    for at in [
        "2026-10-04T24:00:00Z",
        "2026-10-04T23:60:00Z",
        "2026-10-04T23:59:60Z",
        "2026-00-04T00:00:00Z",
        "2026-13-04T00:00:00Z",
        "2026-10-00T00:00:00Z",
        "2026-10-32T00:00:00Z",
        "2026-10-04T00:00:00z",
        "2026-10-04t00:00:00Z",
        "2026-10-04 00:00:00Z",
        "2026-10-04T00:00:00+00:00",
        "2026-10-04T02:00:00+02:00",
        "2026-10-04T00:00:00.000Z",
        "2026-10-04T00:00:00.5Z",
        "２026-10-04T00:00:00Z",
    ] {
        let text = smoke_with("at: \"2026-10-04T00:00:00Z\"", &format!("at: \"{at}\""));
        let error = Fixture::from_yaml(&text)
            .map(|_| ())
            .expect_err(&format!("`{at}` is refused"));
        assert_eq!(
            error.to_string(),
            format!("evaluation instant `{at}` is not written YYYY-MM-DDTHH:MM:SSZ")
        );
    }
}

/// The last instant of a year loads, and evidence observed exactly at the instant is "not after"
/// it, as the format documents. Kills `(1..12)`, `(1..31)`, `< 59` and `>=` mutants.
#[test]
fn adversary_harness_instant_edges_that_load() {
    let text = smoke_with(
        "at: \"2026-10-04T00:00:00Z\"",
        "at: \"2026-12-31T23:59:59Z\"",
    )
    .replace(
        "observed_at: \"2026-10-03T23:00:00Z\"",
        "observed_at: \"2026-12-31T23:59:59Z\"",
    );
    let fixture = Fixture::from_yaml(&text).unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(fixture.at(), "2026-12-31T23:59:59Z");
}
