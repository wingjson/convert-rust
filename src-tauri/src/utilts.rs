use std::path::{Path, PathBuf};


pub fn get_target(path_str: &str, target_type: i32) -> Option<String> {
    let path = Path::new(path_str);
    if let Some(parent) = path.parent() {
        let mut new_path = PathBuf::from(parent);
        if let Some(stem) = path.file_stem() {
            new_path.push(stem);
            new_path.set_extension(get_ext(target_type)?);
            // 将 PathBuf 转换为 String
            return new_path.to_str().map(|s| s.replace("\\", "/"));
        }
    }
    None
}


fn get_ext(target_type: i32) -> Option<String> {
    match target_type {
        1 => Some("pdf".to_string()),
        2 => Some("docx".to_string()),
        3 => Some("xlsx".to_string()),
        4 => Some("pptx".to_string()),
        _ => None, // 如果不匹配任何已知类型，则返回 None
    }
}

#[test]
fn test() {
    let path_str = "/path/to/your/file.txt";
    if let Some(new_path) = get_target(path_str,3) {
        println!("Path without file extension: {}", new_path);
    } else {
        println!("Could not process the path");
    }
}