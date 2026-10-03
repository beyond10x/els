#![forbid(unsafe_code)]

//! Bootstrap engineering-domain layer over Canon.

use b10x_canon::ProtocolId;

pub const SOFTWARE_CHANGE_V1: &str = "software.change/1";
pub const INCIDENT_RESPONSE_V1: &str = "incident.response/1";

pub fn software_change_protocol() -> ProtocolId {
    ProtocolId(SOFTWARE_CHANGE_V1.to_owned())
}

pub fn incident_response_protocol() -> ProtocolId {
    ProtocolId(INCIDENT_RESPONSE_V1.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engineering_protocol_ids_are_stable() {
        assert_eq!(software_change_protocol().0, "software.change/1");
        assert_eq!(incident_response_protocol().0, "incident.response/1");
    }
}
