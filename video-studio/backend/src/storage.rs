use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Storage backend (local filesystem for VPS, easily extensible to S3)
pub struct StorageBackend {
    base_path: PathBuf,
}

impl StorageBackend {
    pub async fn new() -> Result<Self> {
        let base_path = std::env::var("STORAGE_PATH")
            .unwrap_or_else(|_| "/var/video-studio/storage".to_string());

        let base_path = PathBuf::from(base_path);

        // Create base directories
        fs::create_dir_all(&base_path).await?;
        fs::create_dir_all(base_path.join("videos")).await?;
        fs::create_dir_all(base_path.join("audio")).await?;
        fs::create_dir_all(base_path.join("images")).await?;
        fs::create_dir_all(base_path.join("renders")).await?;
        fs::create_dir_all(base_path.join("proxies")).await?;
        fs::create_dir_all(base_path.join("compositions")).await?;
        fs::create_dir_all(base_path.join("templates")).await?;

        Ok(Self { base_path })
    }

    /// Save uploaded file
    pub async fn save_file(&self, file_type: &str, filename: &str, data: &[u8]) -> Result<String> {
        let file_path = self.base_path.join(file_type).join(filename);

        let mut file = fs::File::create(&file_path).await?;
        file.write_all(data).await?;
        file.flush().await?;

        Ok(file_path.to_string_lossy().to_string())
    }

    /// Get file path
    pub fn get_path(&self, file_type: &str, filename: &str) -> PathBuf {
        self.base_path.join(file_type).join(filename)
    }

    /// Delete file
    pub async fn delete_file(&self, file_path: &str) -> Result<()> {
        fs::remove_file(file_path).await?;
        Ok(())
    }

    /// Generate thumbnail for video
    pub async fn generate_thumbnail(&self, video_path: &str, output_name: &str) -> Result<String> {
        use std::process::Command;

        let thumbnail_path = self.base_path.join("images").join(format!("{}.jpg", output_name));

        let output = Command::new("ffmpeg")
            .args(&[
                "-i", video_path,
                "-ss", "00:00:01",
                "-vframes", "1",
                "-vf", "scale=320:-1",
                thumbnail_path.to_str().unwrap()
            ])
            .output()?;

        if !output.status.success() {
            anyhow::bail!("Thumbnail generation failed");
        }

        Ok(thumbnail_path.to_string_lossy().to_string())
    }
}
