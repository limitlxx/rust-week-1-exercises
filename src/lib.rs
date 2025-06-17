// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Input too short to contain a version".into());
    }

    let version_bytes = hex::decode(&raw_tx_hex[0..8])
        .map_err(|_| "Invalid hex in version field")?;

    if version_bytes.len() != 4 {
        return Err("Version field must be exactly 4 bytes".into());
    }

    Ok(u32::from_le_bytes(version_bytes.try_into().unwrap()))
}

