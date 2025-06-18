use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{Result, McpImageError};
use crate::converters::{AutoConverter, ImageConverter, ImageMagickConverter, DarktableConverter};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertDngToWebpArgs {
    pub input_path: String,
    pub output_path: String,
    #[serde(default = "default_converter")]
    pub converter: String,
}

fn default_converter() -> String {
    "auto".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckConvertersArgs {}

#[derive(Debug, Serialize)]
pub struct ConversionResult {
    pub success: bool,
    pub message: String,
    pub output_path: String,
}

#[derive(Debug, Serialize)]
pub struct ConverterInfo {
    pub name: String,
    pub available: bool,
}

#[derive(Debug, Serialize)]
pub struct CheckConvertersResult {
    pub converters: Vec<ConverterInfo>,
    pub available_count: usize,
}

pub struct ImageHandler {
    auto_converter: AutoConverter,
    imagemagick: ImageMagickConverter,
    darktable: DarktableConverter,
}

impl ImageHandler {
    pub fn new() -> Self {
        Self {
            auto_converter: AutoConverter::new(),
            imagemagick: ImageMagickConverter::new(),
            darktable: DarktableConverter::new(),
        }
    }
    
    pub async fn convert_dng_to_webp(&self, args: ConvertDngToWebpArgs) -> Result<Value> {
        let input_path = PathBuf::from(&args.input_path);
        let output_path = PathBuf::from(&args.output_path);
        
        // Select converter based on user preference
        let converter: &dyn ImageConverter = match args.converter.as_str() {
            "imagemagick" => {
                if !self.imagemagick.is_available() {
                    return Err(McpImageError::ConverterNotAvailable(
                        "ImageMagick is not available".to_string()
                    ));
                }
                &self.imagemagick
            },
            "darktable" => {
                if !self.darktable.is_available() {
                    return Err(McpImageError::ConverterNotAvailable(
                        "darktable-cli is not available".to_string()
                    ));
                }
                &self.darktable
            },
            _ => &self.auto_converter,
        };
        
        // Perform conversion
        converter.convert_dng_to_webp(&input_path, &output_path).await?;
        
        let result = ConversionResult {
            success: true,
            message: format!(
                "Successfully converted {} to {} using {}",
                args.input_path,
                args.output_path,
                converter.name()
            ),
            output_path: args.output_path,
        };
        
        Ok(serde_json::to_value(result)?)
    }
    
    pub async fn check_converters(&self, _args: CheckConvertersArgs) -> Result<Value> {
        let converters = vec![
            ConverterInfo {
                name: "imagemagick".to_string(),
                available: self.imagemagick.is_available(),
            },
            ConverterInfo {
                name: "darktable".to_string(),
                available: self.darktable.is_available(),
            },
        ];
        
        let available_count = converters.iter().filter(|c| c.available).count();
        
        let result = CheckConvertersResult {
            converters,
            available_count,
        };
        
        Ok(serde_json::to_value(result)?)
    }
    
    pub fn get_convert_tool_schema() -> Value {
        json!({
            "name": "convert_dng_to_webp",
            "description": "Convert DNG image to WebP format without loss",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "input_path": {
                        "type": "string",
                        "description": "Path to DNG file"
                    },
                    "output_path": {
                        "type": "string",
                        "description": "Path for WebP output"
                    },
                    "converter": {
                        "type": "string",
                        "enum": ["auto", "imagemagick", "darktable"],
                        "default": "auto",
                        "description": "Which converter to use (auto selects the best available)"
                    }
                },
                "required": ["input_path", "output_path"]
            }
        })
    }
    
    pub fn get_check_tool_schema() -> Value {
        json!({
            "name": "check_converters",
            "description": "Check which image converters are available on the system",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }
    
    pub async fn handle_tool_call(&self, name: &str, arguments: Value) -> Result<Value> {
        match name {
            "convert_dng_to_webp" => {
                let args: ConvertDngToWebpArgs = serde_json::from_value(arguments)
                    .map_err(|e| McpImageError::Mcp(format!("Invalid params: {}", e)))?;
                
                let result = self.convert_dng_to_webp(args).await?;
                
                // Extract the message from the result
                let message = if let Some(msg) = result.get("message").and_then(|m| m.as_str()) {
                    msg.to_string()
                } else {
                    serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Conversion completed".to_string())
                };
                
                // Return in MCP content array format
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": message
                    }]
                }))
            }
            "check_converters" => {
                let args: CheckConvertersArgs = serde_json::from_value(arguments)
                    .map_err(|e| McpImageError::Mcp(format!("Invalid params: {}", e)))?;
                
                let result = self.check_converters(args).await?;
                
                // Format the result as readable text
                let mut text = String::new();
                if let Some(converters) = result.get("converters").and_then(|c| c.as_array()) {
                    text.push_str("Available converters:\n");
                    for converter in converters {
                        if let (Some(name), Some(available)) = (
                            converter.get("name").and_then(|n| n.as_str()),
                            converter.get("available").and_then(|a| a.as_bool())
                        ) {
                            text.push_str(&format!("- {}: {}\n", name, if available { "Available" } else { "Not available" }));
                        }
                    }
                    
                    if let Some(count) = result.get("available_count").and_then(|c| c.as_u64()) {
                        text.push_str(&format!("\nTotal available: {}", count));
                    }
                } else {
                    text = serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Check completed".to_string());
                }
                
                // Return in MCP content array format
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": text
                    }]
                }))
            }
            _ => Err(McpImageError::Mcp(format!(
                "Unknown tool: {}",
                name
            )))
        }
    }
}