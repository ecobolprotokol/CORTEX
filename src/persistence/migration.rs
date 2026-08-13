pub struct MigrationHandler;

impl MigrationHandler {
    pub fn new() -> Self { Self }

    pub fn needs_migration(&self, current_version: u32, target_version: u32) -> bool {
        current_version < target_version
    }

    pub fn migrate(&self, data: &[u8], _from_version: u32, _to_version: u32) -> Vec<u8> {
        data.to_vec()
    }
}
