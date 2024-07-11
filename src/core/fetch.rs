use std::path::PathBuf;

use crate::error::Res;

use super::uri::TemplateURI;

pub trait TemplateFetcher {
    fn fetch(&self, uri: &TemplateURI) -> Res<PathBuf>;
}

pub struct LocalFetcher;

impl TemplateFetcher for LocalFetcher {
    fn fetch(&self, uri: &TemplateURI) -> Res<PathBuf> {
        match uri {
            TemplateURI::Local(path) => Ok(PathBuf::from(path)),
        }
    }
}
