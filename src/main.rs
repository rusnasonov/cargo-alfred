use cargo_alfred::cargo;
use handlebars::Handlebars;
use serde_json::json;
use std::{
    error, fs,
    io::{prelude::*, Write},
};
use zip;

fn build(package_metadata: cargo::CargoMetadata) {
    let reg = Handlebars::new();
    let info_plist_template = String::from_utf8_lossy(include_bytes!("Info.plist.template"));
    let package: &cargo::CargoMetadataPackage = package_metadata.packages.first().unwrap();
    let alfred_metadata: &cargo::AlfredMetadata = &package.metadata.alfred;
    let targets = &package.targets;
    let bin_name = &targets
        .into_iter()
        .filter(|target| target.kind == vec!["bin"])
        .collect::<Vec<&cargo::CargoMetadataTarget>>()
        .first()
        .unwrap()
        .name;
    let bin_path = format!("target/release/{}", bin_name);
    let info_plist = reg
        .render_template(
            &info_plist_template,
            &json!({
                "workflow_name": alfred_metadata.workflow_name,
                "bundle_id": alfred_metadata.bundle_id,
                "category": alfred_metadata.category,
                "created_by": alfred_metadata.created_by,
                "description": alfred_metadata.description,
                "name": alfred_metadata.name,
                "keyword": alfred_metadata.keyword,
                "title": alfred_metadata.title,
                "include": alfred_metadata.include,
                "version": package.version,
                "bin_name": bin_path
            }),
        )
        .unwrap();
    let file =
        fs::File::create(format!("{}.alfredworkflow", alfred_metadata.workflow_name)).unwrap();
    let mut archive = zip::ZipWriter::new(file);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let mut buffer = Vec::new();

    archive.start_file(&bin_path, options.unix_permissions(0o755)).unwrap();
    let mut bin_file = fs::File::open(bin_path).unwrap();
    bin_file.read_to_end(&mut buffer).unwrap();
    archive.write(&*buffer).unwrap();
    buffer.clear();

    archive.start_file("info.plist", options).unwrap();
    archive.write(info_plist.as_bytes()).unwrap();

    for path in &alfred_metadata.include {
        archive.start_file(path, options).unwrap();
        let mut f = fs::File::open(path).unwrap();
        f.read_to_end(&mut buffer).unwrap();
        archive.write(&*buffer).unwrap();
        buffer.clear();
    }
    archive.finish().unwrap();
}

fn main() -> Result<(), Box<dyn error::Error>> {
    cargo::build()?;
    let metadata = cargo::metadata()?;
    build(metadata);
    Ok(())
}
