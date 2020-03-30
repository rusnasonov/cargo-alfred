use serde_derive::{Deserialize, Serialize};
use std::{
    process::Command,
    error,
};


#[derive(Debug, Deserialize)]
pub struct CargoMetadata {
    pub packages: Vec<CargoMetadataPackage>,
    pub resolve: Option<CargoMetadataResolve>,
    #[serde(default)]
    pub workspace_members: Vec<String>,
    pub target_directory: String,
}

#[derive(Debug, Deserialize)]
pub struct CargoMetadataResolve {
    pub root: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CargoMetadataPackage {
    pub id: String,
    pub name: String,
    pub targets: Vec<CargoMetadataTarget>,
    pub manifest_path: String,
    pub metadata: Metadata
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub alfred: AlfredMetadata,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AlfredMetadata {
    pub workflow_name: String,
    pub bundle_id: String,
    pub category: String,
    pub created_by: String,
    pub description: String,
    pub name: String,
    pub keyword: String,
    pub title: String,
    pub include: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CargoMetadataTarget {
    pub name: String,
    pub kind: Vec<String>,
    pub crate_types: Vec<String>,
}

pub fn build() -> Result<(), Box<dyn error::Error>> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    cmd.arg("--release");
    cmd.output()?;
    Ok(())
}

pub fn metadata() -> Result<CargoMetadata, Box<dyn error::Error>> {
    let mut cmd = Command::new("cargo");
    cmd.arg("metadata");
    cmd.arg("--format-version=1");
    cmd.arg("--no-deps");
    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let metadata = serde_json::from_str(&stdout)
        .or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>));
    metadata
}
