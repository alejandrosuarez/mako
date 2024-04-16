use mako_core::anyhow::{anyhow, Result};
use mako_core::base64::engine::general_purpose;
use mako_core::base64::Engine;
use mako_core::regex::Regex;

pub fn base64_encode<T: AsRef<[u8]>>(raw: T) -> String {
    general_purpose::STANDARD.encode(raw)
}

pub fn base64_decode(bytes: &[u8]) -> Vec<u8> {
    general_purpose::STANDARD.decode(bytes).unwrap()
}

pub trait ParseRegex {
    fn parse_into_regex(&self) -> Result<Option<Regex>>;
}

impl ParseRegex for Option<String> {
    fn parse_into_regex(&self) -> Result<Option<Regex>> {
        self.as_ref().map_or(Ok(None), |v| {
            Regex::new(v)
                .map(Some)
                .map_err(|_| anyhow!("Config Error invalid regex: {}", v))
        })
    }
}
