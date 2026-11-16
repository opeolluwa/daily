use ulid::Ulid;
use uuid::Uuid;

/// Generate a new 26-character Crockford Base32 ULID.
pub fn new_id() -> String {
    Ulid::generate().to_string()
}

/// Deterministically convert a UUID's 128-bit value into a ULID string.
pub fn uuid_to_ulid(value: Uuid) -> String {
    Ulid::from_bytes(*value.as_bytes()).to_string()
}

/// Recover the UUID whose 128-bit value is encoded by a ULID.
pub fn ulid_to_uuid(value: &str) -> Option<Uuid> {
    value
        .parse::<Ulid>()
        .ok()
        .map(|ulid| Uuid::from_bytes(ulid.to_bytes()))
}

/// Canonicalize an identifier for comparisons and storage.
pub fn normalize_id(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}

/// Whether a string is a valid 26-character ULID.
pub fn is_ulid(value: &str) -> bool {
    value.parse::<Ulid>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_uuid_maps_to_zero_ulid() {
        let uuid = Uuid::nil();
        assert_eq!(uuid_to_ulid(uuid), "00000000000000000000000000");
    }

    #[test]
    fn max_uuid_maps_to_max_ulid() {
        let uuid = Uuid::from_bytes([0xff; 16]);
        assert_eq!(uuid_to_ulid(uuid), "7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
    }

    #[test]
    fn default_workspace_mapping_matches_sql() {
        let uuid = Uuid::parse_str("85b49460-f44a-47f4-930a-8d0b45ec34be").unwrap();
        assert_eq!(uuid_to_ulid(uuid), "45PJA61X2A8ZT962MD1D2YRD5Y");
        assert_eq!(ulid_to_uuid("45PJA61X2A8ZT962MD1D2YRD5Y"), Some(uuid));
    }

    #[test]
    fn uuid_round_trips_through_ulid() {
        let uuid = Uuid::new_v4();
        let ulid = uuid_to_ulid(uuid);
        assert_eq!(ulid_to_uuid(&ulid), Some(uuid));
        assert!(is_ulid(&ulid));
    }

    #[test]
    fn normalize_id_is_uppercase_and_trimmed() {
        assert_eq!(normalize_id("  abc123  "), "ABC123");
    }
}
