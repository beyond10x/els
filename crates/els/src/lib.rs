#![forbid(unsafe_code)]

//! Bootstrap engineering-domain layer over Canon.

use b10x_canon::model::ProtocolId;

pub mod vocabulary;

pub const SOFTWARE_CHANGE_V1: &str = "software.change/1";
pub const INCIDENT_RESPONSE_V1: &str = "incident.response/1";

pub fn software_change_protocol() -> ProtocolId {
    ProtocolId::new(SOFTWARE_CHANGE_V1)
}

pub fn incident_response_protocol() -> ProtocolId {
    ProtocolId::new(INCIDENT_RESPONSE_V1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engineering_protocol_ids_are_stable() {
        assert_eq!(software_change_protocol().as_str(), "software.change/1");
        assert_eq!(incident_response_protocol().as_str(), "incident.response/1");
    }
}
