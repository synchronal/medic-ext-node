use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct DependencyVersions {
    pub current: String,
    pub latest: String,
}

#[derive(Debug, Deserialize)]
pub struct OutdatedInfo {
    #[serde(flatten)]
    pub dependencies: HashMap<String, DependencyVersions>,
}

impl OutdatedInfo {
    pub fn any(&self) -> bool {
        !self.dependencies.is_empty()
    }
}
