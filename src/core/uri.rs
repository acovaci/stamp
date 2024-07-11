use std::path::PathBuf;

use crate::error::Res;

use super::fetch::{LocalFetcher, TemplateFetcher};

pub enum TemplateURI {
    Local(String),
}

impl TemplateURI {
    pub fn fetch(&self) -> Res<PathBuf> {
        match self {
            TemplateURI::Local(_) => LocalFetcher.fetch(self),
        }
    }
}

impl From<&str> for TemplateURI {
    fn from(s: &str) -> Self {
        TemplateURI::Local(s.to_string())
    }
}

impl From<String> for TemplateURI {
    fn from(s: String) -> Self {
        TemplateURI::Local(s)
    }
}
