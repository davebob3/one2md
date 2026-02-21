use std::path::Path;

/// Replace characters that are invalid in directory/file names with `_`.
pub fn cook_name(name: &str) -> String {
    let cooked: String = name
        .chars()
        .map(|c| {
            if is_invalid_filename_char(c) {
                '_'
            } else {
                c
            }
        })
        .collect();
    let trimmed = cooked.trim().to_string();
    let trimmed = trimmed.trim_matches('_').to_string();
    if trimmed.is_empty() {
        "_".to_string()
    } else {
        trimmed
    }
}

/// Generate a unique directory name under `parent` by appending `_N` if needed.
pub fn unique_dir_name(base: &str, parent: &Path) -> String {
    let candidate = parent.join(base);
    if !candidate.exists() {
        return base.to_string();
    }
    let mut i = 1;
    loop {
        let name = format!("{}_{}", base, i);
        if !parent.join(&name).exists() {
            return name;
        }
        i += 1;
    }
}

/// Generate a unique file name under `dir` by appending `_N` before the extension.
pub fn unique_file_name(filename: &str, dir: &Path) -> String {
    let candidate = dir.join(filename);
    if !candidate.exists() {
        return filename.to_string();
    }

    let path = Path::new(filename);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or(filename);
    let ext = path.extension().and_then(|s| s.to_str());

    let mut i = 1;
    loop {
        let name = match ext {
            Some(e) => format!("{}_{}.{}", stem, i, e),
            None => format!("{}_{}", stem, i),
        };
        if !dir.join(&name).exists() {
            return name;
        }
        i += 1;
    }
}

fn is_invalid_filename_char(c: char) -> bool {
    matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' | ' ')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_cook_name_simple() {
        assert_eq!(cook_name("Hello World"), "Hello_World");
    }

    #[test]
    fn test_cook_name_invalid_chars() {
        assert_eq!(cook_name("file/name:test"), "file_name_test");
        assert_eq!(cook_name("a*b?c"), "a_b_c");
        assert_eq!(cook_name("a<b>c"), "a_b_c");
    }

    #[test]
    fn test_cook_name_empty() {
        assert_eq!(cook_name(""), "_");
        assert_eq!(cook_name("   "), "_");
    }

    #[test]
    fn test_cook_name_preserves_unicode() {
        assert_eq!(cook_name("café résumé"), "café_résumé");
    }

    #[test]
    fn test_unique_dir_name_no_conflict() {
        let dir = tempdir().unwrap();
        assert_eq!(unique_dir_name("mydir", dir.path()), "mydir");
    }

    #[test]
    fn test_unique_dir_name_with_conflict() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("mydir")).unwrap();
        assert_eq!(unique_dir_name("mydir", dir.path()), "mydir_1");
    }

    #[test]
    fn test_unique_dir_name_multiple_conflicts() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("mydir")).unwrap();
        fs::create_dir(dir.path().join("mydir_1")).unwrap();
        assert_eq!(unique_dir_name("mydir", dir.path()), "mydir_2");
    }

    #[test]
    fn test_unique_file_name_no_conflict() {
        let dir = tempdir().unwrap();
        assert_eq!(unique_file_name("image.png", dir.path()), "image.png");
    }

    #[test]
    fn test_unique_file_name_with_conflict() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("image.png"), b"").unwrap();
        assert_eq!(unique_file_name("image.png", dir.path()), "image_1.png");
    }

    #[test]
    fn test_unique_file_name_no_extension() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("readme"), b"").unwrap();
        assert_eq!(unique_file_name("readme", dir.path()), "readme_1");
    }
}
