use async_trait::async_trait;
use std::path::Path;
use tokio::process::Command;
use which::which;

use crate::{Result, McpImageError};
use super::ImageConverter;

pub struct ImageMagickConverter {
    command: String,
}

impl ImageMagickConverter {
    pub fn new() -> Self {
        // Try to find convert7 first, then fall back to magick
        let command = if which("convert7").is_ok() {
            "convert7".to_string()
        } else if which("magick").is_ok() {
            "magick".to_string()
        } else {
            "convert7".to_string() // Default, will fail in is_available
        };
        
        Self { command }
    }
}

#[async_trait]
impl ImageConverter for ImageMagickConverter {
    async fn convert_dng_to_webp(&self, input: &Path, output: &Path) -> Result<()> {
        // Validate input file exists
        if !input.exists() {
            return Err(McpImageError::FileNotFound(
                input.display().to_string()
            ));
        }
        
        // Ensure input has .dng extension
        if input.extension().and_then(|s| s.to_str()) != Some("dng") &&
           input.extension().and_then(|s| s.to_str()) != Some("DNG") {
            return Err(McpImageError::InvalidInput(
                "Input file must be a DNG file".to_string()
            ));
        }
        
        // Create output directory if it doesn't exist
        if let Some(parent) = output.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // Build and execute the command
        let output_result = Command::new(&self.command)
            .arg(input)
            .arg("-define")
            .arg("webp:lossless=true")
            .arg("-define")
            .arg("webp:exact=true")
            .arg("-define")
            .arg("webp:method=6")
            .arg("-define")
            .arg("webp:partition-limit=0")
            .arg(output)
            .output()
            .await?;
        
        if output_result.status.success() {
            tracing::info!(
                "Successfully converted {} to {}",
                input.display(),
                output.display()
            );
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            Err(McpImageError::ConversionFailed(format!(
                "ImageMagick conversion failed: {}",
                stderr
            )))
        }
    }
    
    fn is_available(&self) -> bool {
        which(&self.command).is_ok()
    }
    
    fn name(&self) -> &'static str {
        "imagemagick"
    }
    
    fn priority(&self) -> u8 {
        60 // Higher priority than darktable (faster)
    }
}