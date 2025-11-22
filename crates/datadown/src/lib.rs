mod ast;
mod parse;
mod parse_minified;

use thiserror::Error;
use serde::Serialize;

pub use ast::{Node, NodeOrString, MinifiedNode};
pub use parse::parse_markdown;
pub use parse_minified::parse_markdown_minified;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat { 
    Json, 
    Yaml, 
    Toml, 
    Xml 
}
impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "json" => Some(Self::Json),
            "yaml" | "yml" => Some(Self::Yaml),
            "toml" => Some(Self::Toml),
            "xml"  => Some(Self::Xml),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ParsingMode {
    #[default]
    Document,
    Minified,
}

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("unsupported format")]
    UnsupportedFormat,
    #[error("serialization: {0}")]
    Ser(String),
}

pub fn convert_str(input: &str, fmt: OutputFormat, mode: ParsingMode) -> Result<String, ConvertError> {
    match mode {
        ParsingMode::Document => {
            let ast = parse_markdown(input);
            serialize_ast(&ast, fmt)
        },
        ParsingMode::Minified => {
            let ast = parse_markdown_minified(input);
            serialize_ast(&ast, fmt)
        },
    }
}

fn serialize_ast<T: Serialize>(ast: &T, fmt: OutputFormat) -> Result<String, ConvertError> {
    match fmt {
        OutputFormat::Json => serde_json::to_string_pretty(&ast)
            .map_err(|e| ConvertError::Ser(e.to_string())),

        #[cfg(feature="yaml")]
        OutputFormat::Yaml => serde_yaml::to_string(&ast)
            .map_err(|e| ConvertError::Ser(e.to_string())),

        #[cfg(not(feature="yaml"))]
        OutputFormat::Yaml => Err(ConvertError::UnsupportedFormat),

        #[cfg(feature="toml")]
        OutputFormat::Toml => toml::to_string_pretty(&ast)
            .map_err(|e| ConvertError::Ser(e.to_string())),

        #[cfg(not(feature="toml"))]
        OutputFormat::Toml => Err(ConvertError::UnsupportedFormat),

        #[cfg(feature="xml")]
        OutputFormat::Xml => quick_xml::se::to_string(&ast)
            .map_err(|e| ConvertError::Ser(e.to_string())),

        #[cfg(not(feature="xml"))]
        OutputFormat::Xml => Err(ConvertError::UnsupportedFormat),
    }
}