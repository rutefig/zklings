use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub fn change_extension(file_name: &OsStr, new_extension: &str) -> PathBuf {
    let path = Path::new(file_name);
    let stem = path.file_stem().unwrap_or(OsStr::new(""));
    PathBuf::from(stem).with_extension(new_extension)
}

pub fn change_extension_with_suffix(
    file_name: &OsStr,
    suffix: &str,
    new_extension: &str,
) -> PathBuf {
    let path = Path::new(file_name);
    let stem = path.file_stem().unwrap_or(OsStr::new(""));
    let new_stem = format!("{}{}", stem.to_str().unwrap_or(""), suffix);
    PathBuf::from(new_stem).with_extension(new_extension)
}

pub fn append_compiled_folder(base_path: &Path, new_folder_name: &str) -> PathBuf {
    base_path.join(format!("{}", new_folder_name))
}
