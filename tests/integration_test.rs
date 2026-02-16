//! Integration tests for one2md.
//!
//! These tests exercise the full conversion pipeline. They require OneNote test
//! files in `test_in/` (which is gitignored). Tests that depend on external
//! files are skipped gracefully when the files are absent.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn binary_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_BIN_EXE_one2md"));
    path
}

fn test_input_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_in")
}

fn test_output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_output")
}

/// Clean and recreate the output directory for a specific test.
fn setup_output_dir(name: &str) -> PathBuf {
    let dir = test_output_dir().join(name);
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("Failed to clean output dir");
    }
    fs::create_dir_all(&dir).expect("Failed to create output dir");
    dir
}

/// Check if a test .one file is available.
fn find_test_section() -> Option<PathBuf> {
    let dir = test_input_dir();
    if !dir.exists() {
        return None;
    }
    // Look for any .one file
    fs::read_dir(&dir)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().map_or(false, |ext| ext == "one"))
}

fn find_test_notebook() -> Option<PathBuf> {
    let dir = test_input_dir();
    if !dir.exists() {
        return None;
    }
    fs::read_dir(&dir)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().map_or(false, |ext| ext == "onetoc2"))
}

#[test]
fn test_help_flag() {
    let output = Command::new(binary_path())
        .arg("--help")
        .output()
        .expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("one2md"), "Help should contain program name");
    assert!(stdout.contains("--filename"), "Help should mention --filename");
    assert!(
        stdout.contains("--destination-directory"),
        "Help should mention --destination-directory"
    );
}

#[test]
fn test_version_flag() {
    let output = Command::new(binary_path())
        .arg("--version")
        .output()
        .expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("one2md"),
        "Version output should contain program name"
    );
}

#[test]
fn test_missing_filename_error() {
    let output = Command::new(binary_path())
        .output()
        .expect("Failed to run binary");

    assert!(
        !output.status.success(),
        "Should fail without required --filename"
    );
}

#[test]
fn test_invalid_extension_error() {
    let output = Command::new(binary_path())
        .args(["--filename", "test.txt"])
        .output()
        .expect("Failed to run binary");

    assert!(!output.status.success(), "Should fail with invalid extension");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unsupported file extension"),
        "Should report unsupported extension"
    );
}

#[test]
fn test_convert_section_dry_run() {
    let input = match find_test_section() {
        Some(p) => p,
        None => {
            eprintln!("Skipping test_convert_section_dry_run: no .one file in test_in/");
            return;
        }
    };

    let out_dir = setup_output_dir("section_dry_run");

    let output = Command::new(binary_path())
        .args([
            "--filename",
            input.to_str().unwrap(),
            "--destination-directory",
            out_dir.to_str().unwrap(),
            "--dry-run",
        ])
        .output()
        .expect("Failed to run binary");

    assert!(
        output.status.success(),
        "Dry run should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // In dry-run mode, no directories should be created beyond the output dir itself
    let entries: Vec<_> = fs::read_dir(&out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(
        entries.is_empty(),
        "Dry run should not create any files or directories"
    );
}

#[test]
fn test_convert_section_full() {
    let input = match find_test_section() {
        Some(p) => p,
        None => {
            eprintln!("Skipping test_convert_section_full: no .one file in test_in/");
            return;
        }
    };

    let out_dir = setup_output_dir("section_full");

    let output = Command::new(binary_path())
        .args([
            "--filename",
            input.to_str().unwrap(),
            "--destination-directory",
            out_dir.to_str().unwrap(),
            "--should-overwrite",
        ])
        .output()
        .expect("Failed to run binary");

    assert!(
        output.status.success(),
        "Full convert should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify: a section directory should exist
    let section_dirs: Vec<_> = fs::read_dir(&out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map_or(false, |t| t.is_dir()))
        .collect();
    assert!(
        !section_dirs.is_empty(),
        "Should have created at least one section directory"
    );

    // Verify: section directory should contain README.md
    let section_dir = &section_dirs[0].path();
    let section_readme = section_dir.join("README.md");
    assert!(
        section_readme.exists(),
        "Section directory should contain README.md"
    );

    let readme_content = fs::read_to_string(&section_readme).unwrap();
    assert!(
        readme_content.starts_with("# "),
        "Section README should start with a heading"
    );

    // Verify: page directories should exist
    let page_dirs: Vec<_> = fs::read_dir(section_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map_or(false, |t| t.is_dir()))
        .collect();
    assert!(
        !page_dirs.is_empty(),
        "Section should contain at least one page directory"
    );

    // Verify: each page directory should have README.md
    for page_dir in &page_dirs {
        let page_readme = page_dir.path().join("README.md");
        assert!(
            page_readme.exists(),
            "Page directory {:?} should contain README.md",
            page_dir.path()
        );
    }
}

#[test]
fn test_convert_notebook_full() {
    let input = match find_test_notebook() {
        Some(p) => p,
        None => {
            eprintln!("Skipping test_convert_notebook_full: no .onetoc2 file in test_in/");
            return;
        }
    };

    let out_dir = setup_output_dir("notebook_full");

    let output = Command::new(binary_path())
        .args([
            "--filename",
            input.to_str().unwrap(),
            "--destination-directory",
            out_dir.to_str().unwrap(),
            "--should-overwrite",
        ])
        .output()
        .expect("Failed to run binary");

    assert!(
        output.status.success(),
        "Full convert should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify: a section directory should exist
    let section_dirs: Vec<_> = fs::read_dir(&out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map_or(false, |t| t.is_dir()))
        .collect();
    assert!(
        !section_dirs.is_empty(),
        "Should have created at least one section directory"
    );

    // Verify: section directory should contain README.md
    let section_dir = &section_dirs[0].path();
    let section_readme = section_dir.join("README.md");
    assert!(
        section_readme.exists(),
        "Section directory should contain README.md"
    );

    let readme_content = fs::read_to_string(&section_readme).unwrap();
    assert!(
        readme_content.starts_with("# "),
        "Section README should start with a heading"
    );

    // Verify: page directories should exist
    let page_dirs: Vec<_> = fs::read_dir(section_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map_or(false, |t| t.is_dir()))
        .collect();
    assert!(
        !page_dirs.is_empty(),
        "Section should contain at least one page directory"
    );

    // Verify: each page directory should have README.md
    for page_dir in &page_dirs {
        let page_readme = page_dir.path().join("README.md");
        assert!(
            page_readme.exists(),
            "Page directory {:?} should contain README.md",
            page_dir.path()
        );
    }
}

#[test]
fn test_overwrite_protection() {
    let input = match find_test_section() {
        Some(p) => p,
        None => {
            eprintln!("Skipping test_overwrite_protection: no .one file in test_in/");
            return;
        }
    };

    let out_dir = setup_output_dir("overwrite_protect");

    // First run with overwrite
    let output = Command::new(binary_path())
        .args([
            "--filename",
            input.to_str().unwrap(),
            "--destination-directory",
            out_dir.to_str().unwrap(),
            "--should-overwrite",
        ])
        .output()
        .expect("Failed to run binary");
    assert!(output.status.success(), "First run should succeed");

    // Second run without overwrite should fail
    let output = Command::new(binary_path())
        .args([
            "--filename",
            input.to_str().unwrap(),
            "--destination-directory",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run binary");
    assert!(
        !output.status.success(),
        "Second run without --should-overwrite should fail"
    );
}
