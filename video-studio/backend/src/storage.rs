use anyhow::Result;

pub struct StorageBackend {
    // TODO: Implement S3 storage
}

impl StorageBackend {
    pub async fn new() -> Result<Self> {
        Ok(Self {})
    }
}
