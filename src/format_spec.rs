use serde::{Deserialize, Serialize};

pub const PEXE_VERSION: &str = "0.1.0";

#[derive(Debug, Serialize, Deserialize)]
pub struct PexeFileFormat {
    pub pexe_version: String,

    pub name: String,
    pub python_version: String,
    pub tarball: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PexeConfig {
    pub project: Project,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub python_version: String,
    pub includes: Option<Vec<String>>,
}
