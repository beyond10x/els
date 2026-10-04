//! Independent review of `story:incident-response-protocol` (wave 2026-10-04-w8), at 0d4eccc.
//!
//! Contract under check: the library's public `incident_response_protocol()` (`crates/els/src/lib.rs:16`)
//! returns a `b10x_canon::model::ProtocolId`, the type Canon matches a case's `protocol` against
//! (`protocol-mismatch`). The story's domain relation cites `lib.rs` as typing ELS protocol ids
//! with that type. The protocol this story ships declares the id `incident.response`
//! (`protocols/incident-response/1.yaml:4`); the library says `incident.response/1`. A case built
//! from the library's id is refused by Canon for the protocol it names.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use support::Fixture;

const INC_492: &str = "fixtures/incident-response/inc-492.fixture.yaml";

#[test]
fn a_case_naming_the_library_protocol_id_is_evaluated_by_the_shipped_protocol() {
    let library = b10x_els::incident_response_protocol();
    let text = std::fs::read_to_string(support::repo_root().join(INC_492)).expect("inc-492 reads");
    let shipped = "  protocol: incident.response\n";
    assert!(text.contains(shipped), "inc-492's case names the protocol");
    let text = text.replacen(shipped, &format!("  protocol: {}\n", library.as_str()), 1);

    let outcome = Fixture::from_yaml(&text)
        .map_err(|error| error.to_string())
        .and_then(|fixture| {
            let compiled = fixture.compile().map_err(|error| error.to_string())?;
            fixture
                .evaluate(&compiled, "initial")
                .map(|_| ())
                .map_err(|refusal| format!("{}: {refusal}", refusal.code()))
        });
    assert_eq!(
        outcome,
        Ok(()),
        "a case whose protocol is `b10x_els::incident_response_protocol()` (`{}`)",
        library.as_str()
    );
}
