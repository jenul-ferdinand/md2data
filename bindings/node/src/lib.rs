use napi::bindgen_prelude::*;
use napi_derive::napi;
use datadown::{convert_str, OutputFormat, ParsingMode};

#[napi]
pub fn convert(input: String, format: String) -> Result<String> {
    let fmt = OutputFormat::from_str(&format)
        .ok_or_else(|| Error::from_reason("invalid format (use json|yaml|toml|xml)"))?;

    convert_str(&input, fmt, ParsingMode::default())
        .map_err(|e| Error::from_reason(e.to_string()))
}