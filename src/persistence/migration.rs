use crate::error::CortexError;

pub const LATEST_VERSION: u32 = 1;

#[derive(Debug, Clone)]
pub struct MigrationStep {
    pub from_version: u32,
    pub to_version: u32,
    pub description: String,
}

pub struct MigrationHandler {
    pub migrations: Vec<MigrationStep>,
}

impl MigrationHandler {
    pub fn new() -> Self {
        Self {
            migrations: vec![MigrationStep {
                from_version: 0,
                to_version: 1,
                description: "Initial version".into(),
            }],
        }
    }

    pub fn needs_migration(&self, current_version: u32, target_version: u32) -> bool {
        current_version < target_version
    }

    pub fn migrate(
        &self,
        data: &[u8],
        from_version: u32,
        to_version: u32,
    ) -> Result<Vec<u8>, CortexError> {
        if from_version >= to_version {
            return Ok(data.to_vec());
        }

        let mut current = data.to_vec();
        let mut current_version = from_version;

        while current_version < to_version {
            current = self.apply_migration(&current, current_version, current_version + 1)?;
            current_version += 1;
        }

        Ok(current)
    }

    fn apply_migration(
        &self,
        data: &[u8],
        from_version: u32,
        to_version: u32,
    ) -> Result<Vec<u8>, CortexError> {
        let _ = self
            .migrations
            .iter()
            .find(|m| m.from_version == from_version && m.to_version == to_version)
            .ok_or_else(|| {
                CortexError::PersistenceError(format!(
                    "No migration path from {} to {}",
                    from_version, to_version
                ))
            })?;

        match (from_version, to_version) {
            (0, 1) => Ok(data.to_vec()),
            _ => Err(CortexError::PersistenceError(format!(
                "Unsupported migration: {} -> {}",
                from_version, to_version
            ))),
        }
    }

    pub fn detect_version(&self, data: &[u8]) -> Result<u32, CortexError> {
        if data.len() < 8 {
            return Err(CortexError::PersistenceError(
                "Data too short to detect version".into(),
            ));
        }

        let magic = [data[0], data[1], data[2], data[3]];
        if magic != crate::persistence::format::FORMAT_MAGIC {
            return Err(CortexError::PersistenceError(
                "Invalid magic bytes".into(),
            ));
        }

        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        Ok(version)
    }

    pub fn sequential_migrate(
        &self,
        data: &[u8],
        from_version: u32,
    ) -> Result<Vec<u8>, CortexError> {
        self.migrate(data, from_version, LATEST_VERSION)
    }

    pub fn migration_path(&self, from: u32, to: u32) -> Result<Vec<MigrationStep>, CortexError> {
        if from >= to {
            return Ok(Vec::new());
        }

        let mut path = Vec::new();
        let mut current = from;

        while current < to {
            let step = self
                .migrations
                .iter()
                .find(|m| m.from_version == current)
                .ok_or_else(|| {
                    CortexError::PersistenceError(format!(
                        "No migration from version {}",
                        current
                    ))
                })?;
            path.push(step.clone());
            current = step.to_version;
        }

        Ok(path)
    }

    pub fn available_versions(&self) -> Vec<u32> {
        let mut versions: Vec<u32> = self
            .migrations
            .iter()
            .flat_map(|m| vec![m.from_version, m.to_version])
            .collect();
        versions.sort();
        versions.dedup();
        versions
    }
}

impl Default for MigrationHandler {
    fn default() -> Self {
        Self::new()
    }
}
