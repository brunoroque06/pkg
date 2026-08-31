use std::path::Path;

pub fn read_file(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

pub fn write_file(path: &Path, cnt: &str) -> Result<(), String> {
    std::fs::write(path, cnt).map_err(|e| e.to_string())
}
