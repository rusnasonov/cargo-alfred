use std::{
    error,
    fs,
};
use tar;
use cargo_alfred::cargo;

fn build(package_metadata: cargo::CargoMetadata) {
    let package = package_metadata.packages.first().unwrap();
    let alfred_metadata: &cargo::AlfredMetadata = &package.metadata.alfred;
    let file = fs::File::create(format!("{}.alfredworkflow", alfred_metadata.workflow_name)).unwrap();
    let mut archive = tar::Builder::new(file);
    archive.append_path(format!("target/release/{}", package.name)).unwrap();
    archive.append_path("Info.plist").unwrap();
    for path in &alfred_metadata.include {
        archive.append_path(path).unwrap();
    }
}


fn main() -> Result<(), Box<dyn error::Error>> {
    cargo::build()?;
    let metadata = cargo::metadata()?;
    build(metadata);
    Ok(())
}
