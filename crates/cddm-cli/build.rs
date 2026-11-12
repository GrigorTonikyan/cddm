use std::env;
use std::fs;
use std::io;
use std::path::Path;

const FALLBACK_INDEX_HTML: &str = "<!DOCTYPE html><html><head><title>CDDM</title></head><body><div id=\"root\"></div></body></html>";

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let target_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &target_path)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), &target_path)?;
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set by Cargo");
    let out_webui_dist = Path::new(&out_dir).join("webui-dist");

    // Reset staged assets in OUT_DIR so removed frontend files do not linger across builds.
    if out_webui_dist.exists() {
        let _ = fs::remove_dir_all(&out_webui_dist);
    }

    let workspace_webui_dist = Path::new("../../webui/dist");
    if workspace_webui_dist.join("index.html").exists() {
        copy_dir_recursive(workspace_webui_dist, &out_webui_dist)
            .expect("failed to stage webui/dist into OUT_DIR/webui-dist");
    } else {
        fs::create_dir_all(&out_webui_dist)
            .expect("failed to create OUT_DIR/webui-dist fallback directory");
        fs::write(out_webui_dist.join("index.html"), FALLBACK_INDEX_HTML)
            .expect("failed to write fallback index.html into OUT_DIR/webui-dist");
    }
    if Path::new("../../webui").exists() {
        println!("cargo:rerun-if-changed=../../webui/dist");
    }
}
