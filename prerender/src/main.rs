use std::env;
use std::fs;
use std::path::PathBuf;

use leptos::prelude::*;

fn main() {
    let dist_dir = env::args().nth(1).unwrap_or_else(|| "dist".to_string());
    let index_path = PathBuf::from(&dist_dir).join("index.html");

    let html = fs::read_to_string(&index_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", index_path.display()));

    let rendered = site::App().to_html();

    let output = html.replacen("<!--SSG_CONTENT-->", &rendered, 1);
    if output == html {
        panic!("SSG_CONTENT placeholder not found in {}", index_path.display());
    }

    fs::write(&index_path, output)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", index_path.display()));

    println!("prerendered {}", index_path.display());
}
