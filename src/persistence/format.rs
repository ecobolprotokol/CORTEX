use crate::error::CortexError;

pub struct FormatHandler;

impl FormatHandler {
    pub fn new() -> Self { Self }

    pub fn serialize(&self, data: &[u8]) -> Result<Vec<u8>, CortexError> {
        Ok(data.to_vec())
    }

    pub fn deserialize(&self, data: &[u8]) -> Result<Vec<u8>, CortexError> {
        Ok(data.to_vec())
    }

    pub fn compute_checksum(data: &[u8]) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}
