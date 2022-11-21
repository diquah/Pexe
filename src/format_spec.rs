use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PexeFileFormat {
    pub name: String,
    pub python_version: String,
    pub tarball: String
}

#[derive(Serialize, Deserialize)]
pub struct PexeConfig {
    pub project: Project,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub python_version: String,
    pub include: Vec<String>
}