use std::fs::read_to_string;
use std::path::Path;

use base64::alphabet::Alphabet;
use base64::engine::{GeneralPurpose, GeneralPurposeConfig};
use base64::Engine;
use miniz_oxide::deflate::compress_to_vec_zlib;

pub struct PumlUrlCreator {
    b64_engine: GeneralPurpose,
}

enum UrlType {
    Png,
    Svg,
    Pdf,
    Ascii,
}

impl PumlUrlCreator {
    pub fn new() -> eyre::Result<Self> {
        let alphabet =
            Alphabet::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_")?;
        let config = GeneralPurposeConfig::new().with_encode_padding(false);
        let engine = GeneralPurpose::new(&alphabet, config);
        Ok(Self { b64_engine: engine })
    }

    pub fn create_png_url(&self, path: impl AsRef<Path>) -> eyre::Result<String> {
        self.create_url(path, UrlType::Png)
    }

    pub fn create_svg_url(&self, path: impl AsRef<Path>) -> eyre::Result<String> {
        self.create_url(path, UrlType::Svg)
    }

    pub fn create_pdf_url(&self, path: impl AsRef<Path>) -> eyre::Result<String> {
        self.create_url(path, UrlType::Pdf)
    }

    pub fn create_ascii_url(&self, path: impl AsRef<Path>) -> eyre::Result<String> {
        self.create_url(path, UrlType::Ascii)
    }

    fn create_url(&self, path: impl AsRef<Path>, url_type: UrlType) -> eyre::Result<String> {
        let url_type = url_type.as_str();
        let payload = self.encode_file(path)?;
        Ok(format!("http://localhost:8080/{url_type}/{payload}"))
    }

    fn encode_file(&self, path: impl AsRef<Path>) -> eyre::Result<String> {
        let content = read_to_string(path)?;
        let content = content.trim();
        let content = content.strip_prefix("@startuml").unwrap_or(content);
        let content = content.strip_suffix("@enduml").unwrap_or(content);
        let content = content.trim();
        Ok(self.encode(content))
    }

    fn encode(&self, text: &str) -> String {
        let compressed = compress_to_vec_zlib(text.as_bytes(), 10);
        self.b64_engine
            .encode(&compressed[2..(compressed.len() - 4)])
    }
}

impl UrlType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Svg => "svg",
            Self::Pdf => "pdf",
            Self::Ascii => "ascii",
        }
    }
}
