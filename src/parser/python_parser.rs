use super::clean_dir;
use std::path::Path;

pub fn clean_import(import: &str, current_dir: &Path) -> String {
    if import.starts_with('.') {
        return import
            .replacen('.', clean_dir(current_dir).as_str(), 1)
            .replace('.', "/")
            .trim_end_matches('/')
            .to_string();
    }

    import.replace('.', "/").to_string()
}
