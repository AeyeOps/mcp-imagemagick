use async_trait::async_trait;
use std::path::Path;
use tokio::process::Command;
use which::which;

use crate::{Result, McpImageError};
use super::ImageConverter;

pub struct DarktableConverter;

impl DarktableConverter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ImageConverter for DarktableConverter {
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
        let output_result = Command::new("darktable-cli")
            .arg(input)
            .arg(output)
            .output()
            .await?;
        
        if output_result.status.success() {
            tracing::info!(
                "Successfully converted {} to {} using darktable",
                input.display(),
                output.display()
            );
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            let stdout = String::from_utf8_lossy(&output_result.stdout);
            
            // darktable-cli sometimes outputs to stdout instead of stderr
            let error_msg = if stderr.is_empty() {
                stdout.to_string()
            } else {
                stderr.to_string()
            };
            
            Err(McpImageError::ConversionFailed(format!(
                "darktable-cli conversion failed: {}",
                error_msg
            )))
        }
    }
    
    fn is_available(&self) -> bool {
        which("darktable-cli").is_ok()
    }
    
    fn name(&self) -> &'static str {
        "darktable"
    }
    
    fn priority(&self) -> u8 {
        40 // Lower priority than ImageMagick (slower but better for RAW)
    }
}