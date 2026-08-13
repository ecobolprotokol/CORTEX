use crate::error::CortexError;

pub const FORMAT_MAGIC: [u8; 4] = [0x43, 0x58, 0x01, 0x00]; // "CX" + version 1.0
pub const CURRENT_VERSION: u32 = 1;

#[derive(Debug, Clone)]
pub struct FileHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub checksum: [u8; 32],
    pub data_size: u64,
    pub created_at: u64,
}

pub struct FormatHandler {
    pub compression_level: i32,
}

impl FormatHandler {
    pub fn new() -> Self {
        Self {
            compression_level: 3,
        }
    }

    pub fn serialize(&self, data: &[u8]) -> Result<Vec<u8>, CortexError> {
        let checksum = Self::compute_checksum(data);
        let compressed = zstd::encode_all(data, self.compression_level)
            .map_err(|e| CortexError::SerializationError(format!("Compression failed: {}", e)))?;

        let header = FileHeader {
            magic: FORMAT_MAGIC,
            version: CURRENT_VERSION,
            checksum,
            data_size: data.len() as u64,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        let mut output = Vec::new();

        output.extend_from_slice(&header.magic);
        output.extend_from_slice(&header.version.to_le_bytes());
        output.extend_from_slice(&header.checksum);
        output.extend_from_slice(&header.data_size.to_le_bytes());
        output.extend_from_slice(&header.created_at.to_le_bytes());

        output.extend_from_slice(&(compressed.len() as u64).to_le_bytes());
        output.extend_from_slice(&compressed);

        Ok(output)
    }

    pub fn deserialize(&self, data: &[u8]) -> Result<Vec<u8>, CortexError> {
        if data.len() < 56 {
            return Err(CortexError::SerializationError(
                "Data too short for header".into(),
            ));
        }

        let magic = [data[0], data[1], data[2], data[3]];
        if magic != FORMAT_MAGIC {
            return Err(CortexError::SerializationError(
                "Invalid magic bytes".into(),
            ));
        }

        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        if version > CURRENT_VERSION {
            return Err(CortexError::SerializationError(format!(
                "Unsupported version: {}",
                version
            )));
        }

        let mut checksum = [0u8; 32];
        checksum.copy_from_slice(&data[8..40]);

        let data_size = u64::from_le_bytes([
            data[40], data[41], data[42], data[43], data[44], data[45], data[46], data[47],
        ]);

        let _created_at = u64::from_le_bytes([
            data[48], data[49], data[50], data[51], data[52], data[53], data[54], data[55],
        ]);

        let compressed_size = u64::from_le_bytes([
            data[56], data[57], data[58], data[59], data[60], data[61], data[62], data[63],
        ]);

        let compressed_start = 64;
        let compressed_end = compressed_start + compressed_size as usize;

        if compressed_end > data.len() {
            return Err(CortexError::SerializationError(
                "Truncated compressed data".into(),
            ));
        }

        let compressed = &data[compressed_start..compressed_end];

        let decompressed = zstd::decode_all(compressed)
            .map_err(|e| CortexError::SerializationError(format!("Decompression failed: {}", e)))?;

        if decompressed.len() as u64 != data_size {
            return Err(CortexError::SerializationError("Data size mismatch".into()));
        }

        let actual_checksum = Self::compute_checksum(&decompressed);
        if actual_checksum != checksum {
            return Err(CortexError::SerializationError("Checksum mismatch".into()));
        }

        Ok(decompressed)
    }

    pub fn compute_checksum(data: &[u8]) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    pub fn read_header(&self, data: &[u8]) -> Result<FileHeader, CortexError> {
        if data.len() < 56 {
            return Err(CortexError::SerializationError("Data too short".into()));
        }

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&data[0..4]);
        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let mut checksum = [0u8; 32];
        checksum.copy_from_slice(&data[8..40]);
        let data_size = u64::from_le_bytes([
            data[40], data[41], data[42], data[43], data[44], data[45], data[46], data[47],
        ]);
        let created_at = u64::from_le_bytes([
            data[48], data[49], data[50], data[51], data[52], data[53], data[54], data[55],
        ]);

        Ok(FileHeader {
            magic,
            version,
            checksum,
            data_size,
            created_at,
        })
    }

    pub fn estimate_compressed_size(&self, data: &[u8]) -> usize {
        let sample_size = data.len().min(1024);
        if sample_size == 0 {
            return 0;
        }
        let sample = &data[..sample_size];
        if let Ok(compressed) = zstd::encode_all(sample, self.compression_level) {
            let ratio = compressed.len() as f64 / sample_size as f64;
            (data.len() as f64 * ratio) as usize
        } else {
            data.len()
        }
    }

    pub fn save_to_file(&self, path: &str, data: &[u8]) -> Result<(), CortexError> {
        let serialized = self.serialize(data)?;
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    CortexError::PersistenceError(format!("Failed to create directory: {}", e))
                })?;
            }
        }
        let tmp_path = format!("{}.tmp", path);
        std::fs::write(&tmp_path, &serialized).map_err(|e| {
            CortexError::PersistenceError(format!("Failed to write temp file: {}", e))
        })?;
        let file = std::fs::File::open(&tmp_path).map_err(|e| {
            CortexError::PersistenceError(format!("Failed to open for sync: {}", e))
        })?;
        file.sync_all()
            .map_err(|e| CortexError::PersistenceError(format!("Failed to sync: {}", e)))?;
        drop(file);
        std::fs::rename(&tmp_path, path)
            .map_err(|e| CortexError::PersistenceError(format!("Failed to rename: {}", e)))?;
        tracing::debug!(path = %path, bytes = serialized.len(), "State saved to disk");
        Ok(())
    }

    pub fn load_from_file(&self, path: &str) -> Result<Vec<u8>, CortexError> {
        let data = std::fs::read(path).map_err(|e| {
            CortexError::PersistenceError(format!("Failed to read state file: {}", e))
        })?;
        self.deserialize(&data)
    }
}

impl Default for FormatHandler {
    fn default() -> Self {
        Self::new()
    }
}
