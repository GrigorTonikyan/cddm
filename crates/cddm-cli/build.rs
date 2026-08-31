use std::fs;
use std::path::Path;

fn main() {
    // Ensure webui/dist directory and fallback index.html exist so that rust-embed
    // compiles cleanly in any environment (including CI or fresh dev checkouts)
    // even if the webui frontend has not yet been built.
    let webui_dist = Path::new("../../webui/dist");
    if !webui_dist.exists() {
        let _ = fs::create_dir_all(webui_dist);
    }
    let index_html = webui_dist.join("index.html");
    if !index_html.exists() {
        let _ = fs::write(
            &index_html,
            "<!DOCTYPE html><html><head><title>CDDM</title></head><body><div id=\"root\"></div></body></html>",
        );
    }
    println!("cargo:rerun-if-changed=../../webui/dist");
}
