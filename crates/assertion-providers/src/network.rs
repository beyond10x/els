use crate::{Arguments, Collected, ProviderContext, ProviderError, arg, known};
use serde_json::json;
use std::{
    collections::BTreeSet,
    io,
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    sync::atomic::{AtomicU16, Ordering},
    time::Instant,
};
static QUERY_ID: AtomicU16 = AtomicU16::new(1);
pub(super) fn tcp(args: &Arguments, context: &ProviderContext) -> Result<Collected, ProviderError> {
    let host = arg(args, "host")?;
    let port = args
        .get("port")
        .and_then(|v| v.as_u64())
        .and_then(|p| u16::try_from(p).ok())
        .filter(|p| *p > 0)
        .ok_or_else(|| ProviderError::Invalid("port must be between 1 and 65535".into()))?;
    let network = context
        .networks
        .get(arg(args, "network")?)
        .ok_or_else(|| ProviderError::Invalid("network context is not registered".into()))?;
    let address = network
        .hosts
        .get(host)
        .copied()
        .ok_or_else(|| ProviderError::Invalid("host is not bound in network context".into()))?;
    match TcpStream::connect_timeout(&SocketAddr::new(address, port), context.timeout) {
        Ok(_) => known(true),
        Err(e) if e.kind() == io::ErrorKind::ConnectionRefused => known(false),
        Err(e) => Ok(Collected::Unavailable(format!(
            "TCP observation unavailable: {e}"
        ))),
    }
}
/// Query an explicit DNS server, never the ambient resolver. A/AAAA answers retain their minimum
/// TTL. NXDOMAIN is false with TTL zero (a negative response is not replay-fresh by default).
pub(super) fn dns(args: &Arguments, context: &ProviderContext) -> Result<Collected, ProviderError> {
    let host = arg(args, "host")?
        .trim_end_matches('.')
        .to_ascii_lowercase();
    if host.is_empty()
        || host.len() > 253
        || host.split('.').any(|s| {
            s.is_empty()
                || s.len() > 63
                || !s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
                || s.starts_with('-')
                || s.ends_with('-')
        })
    {
        return Err(ProviderError::Invalid(
            "DNS host must be an ASCII DNS name".into(),
        ));
    }
    let resolver: SocketAddr = arg(args, "resolver")?.parse().map_err(|_| {
        ProviderError::Invalid("resolver must be an explicit IP:port socket address".into())
    })?;
    let socket = UdpSocket::bind(if resolver.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    })?;
    socket.connect(resolver)?;
    let deadline = Instant::now() + context.timeout;
    let mut addresses = BTreeSet::new();
    let mut ttl = u32::MAX;
    for kind in [1u16, 28u16] {
        let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
            return Ok(Collected::Unavailable("DNS timeout".into()));
        };
        socket.set_read_timeout(Some(remaining))?;
        socket.set_write_timeout(Some(remaining))?;
        let id = QUERY_ID.fetch_add(1, Ordering::Relaxed);
        let mut query = Vec::new();
        for n in [id, 0x0100, 1, 0, 0, 0] {
            query.extend_from_slice(&n.to_be_bytes());
        }
        for label in host.split('.') {
            query.push(label.len() as u8);
            query.extend_from_slice(label.as_bytes());
        }
        query.push(0);
        query.extend_from_slice(&kind.to_be_bytes());
        query.extend_from_slice(&1u16.to_be_bytes());
        if let Err(e) = socket.send(&query) {
            return Ok(Collected::Unavailable(format!("DNS send unavailable: {e}")));
        }
        let mut bytes = [0u8; 65535];
        let n = match socket.recv(&mut bytes) {
            Ok(n) => n,
            Err(e) => {
                return Ok(Collected::Unavailable(format!(
                    "DNS receive unavailable: {e}"
                )));
            }
        };
        match parse(&bytes[..n], id, &host, kind)? {
            Answer::Unavailable(reason) => return Ok(Collected::Unavailable(reason)),
            Answer::Negative => {
                if !addresses.is_empty() {
                    return Err(ProviderError::Invalid(
                        "DNS answer contradicts earlier positive address observation".into(),
                    ));
                }
                return known(
                    json!({"resolves":false,"ttl_seconds":0,"resolver":resolver.to_string(),"addresses":[]}),
                );
            }
            Answer::Values(found, lifetime) => {
                addresses.extend(found);
                ttl = ttl.min(lifetime);
            }
        }
    }
    known(
        json!({"resolves":!addresses.is_empty(),"ttl_seconds":if ttl==u32::MAX{0}else{ttl},"resolver":resolver.to_string(),"addresses":addresses}),
    )
}
enum Answer {
    Negative,
    Unavailable(String),
    Values(Vec<String>, u32),
}
fn u16_at(bytes: &[u8], at: usize) -> Result<u16, ProviderError> {
    let v = bytes
        .get(at..at + 2)
        .ok_or_else(|| ProviderError::Invalid("truncated DNS response".into()))?;
    Ok(u16::from_be_bytes([v[0], v[1]]))
}
fn name(bytes: &[u8], cursor: &mut usize) -> Result<String, ProviderError> {
    let mut offset = *cursor;
    let mut jumped = false;
    let mut labels = Vec::new();
    let mut steps = 0;
    loop {
        steps += 1;
        if steps > 128 {
            return Err(ProviderError::Invalid(
                "DNS name compression cycle or budget".into(),
            ));
        }
        let length = *bytes
            .get(offset)
            .ok_or_else(|| ProviderError::Invalid("truncated DNS name".into()))?;
        offset += 1;
        if length & 0xc0 == 0xc0 {
            let next = *bytes
                .get(offset)
                .ok_or_else(|| ProviderError::Invalid("truncated DNS pointer".into()))?;
            if !jumped {
                *cursor = offset + 1;
                jumped = true;
            }
            offset = (((length & 0x3f) as usize) << 8) | next as usize;
            continue;
        }
        if length & 0xc0 != 0 {
            return Err(ProviderError::Invalid("invalid DNS label".into()));
        }
        if length == 0 {
            if !jumped {
                *cursor = offset;
            }
            break;
        }
        let label = bytes
            .get(offset..offset + length as usize)
            .ok_or_else(|| ProviderError::Invalid("truncated DNS label".into()))?;
        labels.push(
            std::str::from_utf8(label)
                .map_err(|_| ProviderError::Invalid("non-ASCII DNS label".into()))?
                .to_ascii_lowercase(),
        );
        offset += length as usize;
    }
    let value = labels.join(".");
    if value.len() > 253 {
        return Err(ProviderError::Invalid("DNS name exceeds limit".into()));
    }
    Ok(value)
}
fn parse(bytes: &[u8], id: u16, host: &str, kind: u16) -> Result<Answer, ProviderError> {
    if u16_at(bytes, 0)? != id {
        return Err(ProviderError::Invalid("DNS transaction mismatch".into()));
    }
    let flags = u16_at(bytes, 2)?;
    if flags & 0x8000 == 0 || flags & 0x7800 != 0 || u16_at(bytes, 4)? != 1 {
        return Err(ProviderError::Invalid("invalid DNS response header".into()));
    }
    if flags & 0x0200 != 0 {
        return Ok(Answer::Unavailable(
            "truncated DNS response requires TCP retry".into(),
        ));
    }
    let mut cursor = 12;
    let question = name(bytes, &mut cursor)?;
    if question != host || u16_at(bytes, cursor)? != kind || u16_at(bytes, cursor + 2)? != 1 {
        return Err(ProviderError::Invalid("DNS question mismatch".into()));
    }
    cursor += 4;
    let answer_count = u16_at(bytes, 6)? as usize;
    let count = answer_count + u16_at(bytes, 8)? as usize + u16_at(bytes, 10)? as usize;
    if count > 4096 {
        return Err(ProviderError::Invalid("DNS record budget exceeded".into()));
    }
    let mut records = Vec::new();
    let mut aliases = Vec::new();
    for index in 0..count {
        let owner = name(bytes, &mut cursor)?;
        let rrtype = u16_at(bytes, cursor)?;
        let class = u16_at(bytes, cursor + 2)?;
        let high = u16_at(bytes, cursor + 4)?;
        let low = u16_at(bytes, cursor + 6)?;
        let ttl = ((high as u32) << 16) | low as u32;
        let length = u16_at(bytes, cursor + 8)? as usize;
        cursor += 10;
        let data = bytes
            .get(cursor..cursor + length)
            .ok_or_else(|| ProviderError::Invalid("truncated DNS record".into()))?;
        if class == 1 {
            match rrtype {
                1 if length != 4 => {
                    return Err(ProviderError::Invalid("invalid DNS A record length".into()));
                }
                28 if length != 16 => {
                    return Err(ProviderError::Invalid(
                        "invalid DNS AAAA record length".into(),
                    ));
                }
                2 | 5 | 12 | 6 => {
                    let mut position = cursor;
                    let _ = name(bytes, &mut position)?;
                    if rrtype == 6 {
                        let _ = name(bytes, &mut position)?;
                        position = position
                            .checked_add(20)
                            .ok_or_else(|| ProviderError::Invalid("DNS SOA overflow".into()))?;
                    }
                    if position != cursor + length {
                        return Err(ProviderError::Invalid(
                            "DNS name record length mismatch".into(),
                        ));
                    }
                }
                _ => {}
            }
        }
        if class == 1 && index < answer_count {
            match (rrtype, length) {
                (1, 4) => records.push((
                    owner,
                    IpAddr::from([data[0], data[1], data[2], data[3]]).to_string(),
                    ttl,
                )),
                (28, 16) => {
                    let mut address = [0; 16];
                    address.copy_from_slice(data);
                    records.push((owner, IpAddr::from(address).to_string(), ttl));
                }
                (5, _) => {
                    let mut position = cursor;
                    let target = name(bytes, &mut position)?;
                    if position > cursor + length {
                        return Err(ProviderError::Invalid(
                            "CNAME exceeds record boundary".into(),
                        ));
                    }
                    aliases.push((owner, target, ttl));
                }
                _ => {}
            }
        }
        cursor += length;
    }
    if cursor != bytes.len() {
        return Err(ProviderError::Invalid(
            "DNS trailing unclaimed bytes".into(),
        ));
    }
    match flags & 15 {
        3 => {
            if answer_count != 0 {
                return Err(ProviderError::Invalid(
                    "NXDOMAIN carries contradictory answer records".into(),
                ));
            }
            if flags & 0x0480 == 0 {
                return Ok(Answer::Unavailable(
                    "DNS negative response is neither recursive nor authoritative".into(),
                ));
            }
            return Ok(Answer::Negative);
        }
        0 => {}
        _ => {
            return Ok(Answer::Unavailable(format!(
                "DNS server returned RCODE {}",
                flags & 15
            )));
        }
    }
    let mut reachable = BTreeSet::from([host.to_owned()]);
    let mut ttl = u32::MAX;
    for _ in 0..128 {
        let before = reachable.len();
        for (owner, target, lifetime) in &aliases {
            if reachable.contains(owner) {
                reachable.insert(target.clone());
                ttl = ttl.min(*lifetime);
            }
        }
        if reachable.len() == before {
            break;
        }
    }
    let mut addresses = Vec::new();
    for (owner, address, lifetime) in records {
        if reachable.contains(&owner) {
            addresses.push(address);
            ttl = ttl.min(lifetime);
        }
    }
    if addresses.is_empty() && !aliases.is_empty() {
        return Ok(Answer::Unavailable(
            "DNS alias has no terminal address answer".into(),
        ));
    }
    if addresses.is_empty() && flags & 0x0480 == 0 {
        return Ok(Answer::Unavailable(
            "DNS referral does not establish absent address records".into(),
        ));
    }
    Ok(Answer::Values(addresses, ttl))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negative_dns_packet_must_validate_every_claimed_section() {
        let mut packet = vec![
            0, 7, 0x81, 0x83, 0, 1, 0, 1, 0, 0, 0, 0, 3, b'a', b'p', b'i', 0, 0, 1, 0, 1,
        ];
        assert!(parse(&packet, 7, "api", 1).is_err());
        packet[7] = 0;
        packet[9] = 1;
        assert!(parse(&packet, 7, "api", 1).is_err());
        packet[9] = 0;
        packet[11] = 1;
        assert!(parse(&packet, 7, "api", 1).is_err());
    }
}
