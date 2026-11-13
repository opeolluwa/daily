use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "dreamfs.ts")]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
}

pub fn version() -> String {
    "0.1.0".into()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn list(root: &str) -> Vec<FileInfo> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root) {
        collect_files(entries, &mut files);
    }
    files
}

#[cfg(not(target_arch = "wasm32"))]
fn collect_files(entries: std::fs::ReadDir, files: &mut Vec<FileInfo>) {
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            let size = path.metadata().map(|m| m.len()).unwrap_or(0);
            files.push(FileInfo {
                path: path.to_string_lossy().into_owned(),
                size,
            });
        } else if path.is_dir() {
            if let Ok(sub) = std::fs::read_dir(&path) {
                collect_files(sub, files);
            }
        }
    }
}

pub fn total_size(files: &[FileInfo]) -> u64 {
    files.iter().map(|f| f.size).sum()
}

#[wasm_bindgen]
pub struct DreamFs;

#[wasm_bindgen]
impl DreamFs {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    #[wasm_bindgen]
    pub fn version(&self) -> String {
        version()
    }

    #[wasm_bindgen]
    pub fn total_size(&self, files_js: JsValue) -> Result<u64, JsValue> {
        let files: Vec<FileInfo> =
            serde_wasm_bindgen::from_value(files_js).map_err(crate::utils::js_err)?;
        Ok(total_size(&files))
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[wasm_bindgen]
    pub fn list(&self, root: &str) -> Result<JsValue, JsValue> {
        let files = list(root);
        crate::utils::to_js(&files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_matches() {
        assert_eq!(version(), "0.1.0");
    }

    #[test]
    fn total_size_sums_correctly() {
        let files = vec![
            FileInfo { path: "a.txt".into(), size: 100 },
            FileInfo { path: "b.txt".into(), size: 200 },
        ];
        assert_eq!(total_size(&files), 300);
    }

    #[test]
    fn list_current_dir() {
        let files = list(".");
        assert!(!files.is_empty());
        assert!(total_size(&files) > 0);
    }
}
