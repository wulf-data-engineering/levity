use prost::Message;
use prost_types::FileDescriptorSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to your proto files
    let proto_dir = Path::new("../protocols");

    // Collect all .proto files in the directory
    let protos: Vec<PathBuf> = fs::read_dir(proto_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|ext| ext == "proto").unwrap_or(false))
        .collect();

    if protos.is_empty() {
        panic!("No .proto files found in {}", proto_dir.display());
    }

    // Path to write temporary descriptor set
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let descriptor_path = out_dir.join("protos_descriptor.bin");

    // Run protoc to generate descriptor set
    let mut cmd = Command::new("protoc");
    cmd.arg(format!("-I{}", proto_dir.display()));
    cmd.arg(format!(
        "--descriptor_set_out={}",
        descriptor_path.display()
    ));
    cmd.arg("--include_imports");
    for p in &protos {
        cmd.arg(p);
    }

    let status = cmd.status()?;
    if !status.success() {
        panic!("protoc failed to generate descriptor set");
    }

    // Read descriptor and decode
    let desc_bytes = fs::read(&descriptor_path)?;
    let fds = FileDescriptorSet::decode(&*desc_bytes)?; // prost::Message trait required

    // Recursively collect fully qualified message names
    fn collect_messages(
        package: &str,
        prefix: &str,
        messages: &[prost_types::DescriptorProto],
        out: &mut Vec<String>,
    ) {
        for msg in messages {
            let name = if prefix.is_empty() {
                format!("{}.{}", package, msg.name.as_deref().unwrap_or_default())
            } else {
                format!(
                    "{}.{}.{}",
                    package,
                    prefix,
                    msg.name.as_deref().unwrap_or_default()
                )
            };
            out.push(name.clone());

            if !msg.nested_type.is_empty() {
                let new_prefix = if prefix.is_empty() {
                    msg.name.as_deref().unwrap_or_default().to_string()
                } else {
                    format!("{}.{}", prefix, msg.name.as_deref().unwrap_or_default())
                };
                collect_messages(package, &new_prefix, &msg.nested_type, out);
            }
        }
    }

    let mut all_message_names = Vec::new();
    for file in fds.file {
        let pkg = file.package.unwrap_or_default();
        collect_messages(&pkg, "", &file.message_type, &mut all_message_names);
    }
    
    // Configure prost-build
    let mut config = prost_build::Config::new();
    let derive_attr = r#"#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = "camelCase")]"#;
    for full_name in &all_message_names {
        config.type_attribute(full_name, derive_attr);
    }

    // Compile protos
    let proto_strs: Vec<&str> = protos.iter().map(|p| p.to_str().unwrap()).collect();
    config.compile_protos(&proto_strs, &[proto_dir.to_str().unwrap()])?;

    Ok(())
}
