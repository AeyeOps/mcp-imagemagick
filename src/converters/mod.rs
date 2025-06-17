mod imagemagick;
mod darktable;

pub use imagemagick::ImageMagickConverter;
pub use darktable::DarktableConverter;

use async_trait::async_trait;
use std::path::Path;
use crate::Result;

#[async_trait]
pub trait ImageConverter: Send + Sync {
    /// Convert a DNG file to WebP format
    async fn convert_dng_to_webp(&self, input: &Path, output: &Path) -> Result<()>;
    
    /// Check if this converter is available on the system
    fn is_available(&self) -> bool;
    
    /// Get the name of this converter
    fn name(&self) -> &'static str;
    
    /// Get converter priority (higher = preferred)
    fn priority(&self) -> u8 {
        50
    }
}

/// Auto-select the best available converter
pub struct AutoConverter {
    converters: Vec<Box<dyn ImageConverter>>,
}

impl AutoConverter {
    pub fn new() -> Self {
        let mut converters: Vec<Box<dyn ImageConverter>> = vec![
            Box::new(ImageMagickConverter::new()),
            Box::new(DarktableConverter::new()),
        ];
        
        // Sort by priority (highest first)
        converters.sort_by_key(|c| std::cmp::Reverse(c.priority()));
        
        Self { converters }
    }
    
    pub fn available_converters(&self) -> Vec<&'static str> {
        self.converters
            .iter()
            .filter(|c| c.is_available())
            .map(|c| c.name())
            .collect()
    }
}

#[async_trait]
impl ImageConverter for AutoConverter {
    async fn convert_dng_to_webp(&self, input: &Path, output: &Path) -> Result<()> {
        for converter in &self.converters {
            if converter.is_available() {
                tracing::info!("Using converter: {}", converter.name());
                return converter.convert_dng_to_webp(input, output).await;
            }
        }
        
        Err(crate::McpImageError::ConverterNotAvailable(
            "No image converter available".to_string()
        ))
    }
    
    fn is_available(&self) -> bool {
        self.converters.iter().any(|c| c.is_available())
    }
    
    fn name(&self) -> &'static str {
        "auto"
    }
}