use std::{
    fs::{self},
    path::Path,
    process::{Command, Stdio},
};

use jsonnet_std_docs::StdLib;

const STDLIB_FILE: &str = "stdlib-content.jsonnet";

fn get_stdlib_urls(version: &str) -> Vec<(String, String)> {
    vec![
        (
            STDLIB_FILE.to_string(),
            format!(
                "https://raw.githubusercontent.com/google/jsonnet/{}/doc/_stdlib_gen/stdlib-content.jsonnet",
                version
            ),
        ),
        (
            "html.libsonnet".to_string(),
            format!(
                "https://raw.githubusercontent.com/google/jsonnet/{}/doc/_stdlib_gen/html.libsonnet",
                version
            ),
        ),
    ]
}

fn build_stdlib() {
    // Use gen dir to avoid downloading the file again after each change
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let gen_dir = format!("{root_dir}/gen");
    let gen_path = Path::new(&gen_dir);
    let _ = fs::create_dir(gen_path);
    let urls = get_stdlib_urls("v0.21.0");
    for (name, url) in urls {
        let url_path = gen_path.join(name);
        if !url_path.exists() {
            let content = reqwest::blocking::get(url).unwrap().text().unwrap();
            fs::write(url_path, content).unwrap();
        }
    }

    let empty_lib = "{\"groups\": []}".to_string();
    // TODO: This fails while cross compiling to windows
    let content = Command::new("jsonnet")
        .arg("-J")
        .arg(gen_path.to_str().unwrap())
        .arg("stdlib.jsonnet")
        .stdout(Stdio::piped())
        .output()
        .map_or(empty_lib.clone(), |o| {
            let s = String::from_utf8(o.stdout).unwrap();
            if s.is_empty() { empty_lib } else { s }
        });

    // Convert html to md
    let mut lib: StdLib = serde_json::from_str(&content).unwrap();
    lib.groups.iter_mut().for_each(|group| {
        group.fields.iter_mut().for_each(|func| {
            func.description = htmd::HtmlToMarkdown::new()
                .convert(&func.description)
                .unwrap();
        });
    });

    let out_content = serde_json::to_string(&lib).unwrap();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let stdlib_path = out_path.join("stdlib.json");
    fs::write(stdlib_path.clone(), out_content)
        .unwrap_or_else(|_| panic!("Failed to write stdlib to out path at {:?}", stdlib_path));
}

fn main() {
    build_stdlib();
}
