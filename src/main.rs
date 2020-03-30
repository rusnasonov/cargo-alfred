use std::{
    error,
    fs,
    io::Write,
};
use tar;
use cargo_alfred::cargo;
use handlebars::Handlebars;
use tempfile::NamedTempFile;

fn build(package_metadata: cargo::CargoMetadata) {
    let reg = Handlebars::new();
    let info_plist_template = String::from_utf8_lossy(include_bytes!("Info.plist.template"));
    let package = package_metadata.packages.first().unwrap();
    let alfred_metadata: &cargo::AlfredMetadata = &package.metadata.alfred;
    // TODO: add path to binary and version
    let info_plist = reg.render_template(
        &info_plist_template,
        alfred_metadata,
    ).unwrap();
    let mut info_plist_temp = NamedTempFile::new().unwrap();
    info_plist_temp.write_all(info_plist.as_bytes()).unwrap();
    let file = fs::File::create(format!("{}.alfredworkflow", alfred_metadata.workflow_name)).unwrap();
    let mut archive = tar::Builder::new(file);
    archive.append_path(format!("target/release/{}", package.name)).unwrap();
    archive.append_file("Info.plist", info_plist_temp.as_file_mut()).unwrap();
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
