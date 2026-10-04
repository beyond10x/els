#![forbid(unsafe_code)]

//! Bootstrap engineering-domain layer over Canon.

use b10x_canon::model::ProtocolId;

pub mod vocabulary;

pub const SOFTWARE_CHANGE_V1: &str = "software.change/1";

/// The id `protocols/incident-response/1.yaml` declares, the id a case names. A Canon
/// [`ProtocolId`] holds the id alone; the revision travels beside it, as the protocol header's
/// `revision` ([`INCIDENT_RESPONSE_REVISION`]).
pub const INCIDENT_RESPONSE: &str = "incident.response";

/// The revision `protocols/incident-response/1.yaml` declares.
pub const INCIDENT_RESPONSE_REVISION: u64 = 1;

pub fn software_change_protocol() -> ProtocolId {
    ProtocolId::new(SOFTWARE_CHANGE_V1)
}

/// The id of the shipped `incident.response/1` protocol, as Canon matches a case against it.
pub fn incident_response_protocol() -> ProtocolId {
    ProtocolId::new(INCIDENT_RESPONSE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engineering_protocol_ids_are_stable() {
        assert_eq!(software_change_protocol().as_str(), "software.change/1");
        assert_eq!(incident_response_protocol().as_str(), "incident.response");
        assert_eq!(INCIDENT_RESPONSE_REVISION, 1);
    }

    /// The id and revision are those the shipped protocol declares, read at run time.
    #[test]
    fn the_incident_response_id_is_the_shipped_protocols() {
        let manifest =
            std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
        let path = std::path::Path::new(&manifest).join("../../protocols/incident-response/1.yaml");
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let protocol = b10x_canon::model::parse(&text).unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(protocol.protocol.id, incident_response_protocol());
        assert_eq!(protocol.protocol.revision, INCIDENT_RESPONSE_REVISION);
    }
}
