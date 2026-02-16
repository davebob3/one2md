pub mod name_utils;

use crate::renderers::readme::ReadmeRenderer;
use crate::renderers::RenderContext;
use log::info;
use onenote_parser::section::{Section, SectionEntry};
use onenote_parser::Parser;
use std::fs;
use std::io;
use std::path::Path;

use name_utils::{cook_name, unique_dir_name};

pub struct ConvertOptions {
    pub dry_run: bool,
    pub should_overwrite: bool,
}

/// Top-level entry point: dispatch based on file extension.
pub fn convert(input: &Path, dest: &Path, opts: &ConvertOptions) -> io::Result<()> {
    let parser = Parser::new();

    let ext = input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "onetoc2" => {
            let notebook = parser
                .parse_notebook(input)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
            for entry in notebook.entries() {
                process_section_entry(entry, dest, opts)?;
            }
        }
        "one" => {
            let section = parser
                .parse_section(input)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
            process_section(&section, dest, opts)?;
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported file extension: {}", ext),
            ));
        }
    }

    Ok(())
}

fn process_section_entry(
    entry: &SectionEntry,
    parent_dir: &Path,
    opts: &ConvertOptions,
) -> io::Result<()> {
    match entry {
        SectionEntry::Section(section) => process_section(section, parent_dir, opts),
        SectionEntry::SectionGroup(group) => {
            let dir_name = cook_name(group.display_name());
            let group_dir = parent_dir.join(&dir_name);
            ensure_dir(&group_dir, opts)?;
            for entry in group.entries() {
                process_section_entry(entry, &group_dir, opts)?;
            }
            Ok(())
        }
    }
}

fn process_section(
    section: &Section,
    parent_dir: &Path,
    opts: &ConvertOptions,
) -> io::Result<()> {
    let section_name = section.display_name();
    let dir_name = cook_name(section_name);
    let section_dir = parent_dir.join(&dir_name);
    ensure_dir(&section_dir, opts)?;

    info!("Processing section: {}", section_name);

    // Collect page info for section README
    let mut page_entries: Vec<(String, String)> = Vec::new();
    let mut untitled_count: usize = 0;

    for page_series in section.page_series() {
        for page in page_series.pages() {
            let title = page
                .title_text()
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    untitled_count += 1;
                    format!("Untitled_{}", untitled_count)
                });
            let page_dir_name = cook_name(&title);
            let page_dir_name = unique_dir_name(&page_dir_name, &section_dir);
            page_entries.push((title, page_dir_name));
        }
    }

    // Write section README.md
    let section_readme = generate_section_readme(section_name, &page_entries);
    write_file(&section_dir.join("README.md"), section_readme.as_bytes(), opts)?;

    // Process each page
    let mut page_idx = 0;
    for page_series in section.page_series() {
        for page in page_series.pages() {
            let (ref _title, ref page_dir_name) = page_entries[page_idx];
            let page_dir = section_dir.join(page_dir_name);
            ensure_dir(&page_dir, opts)?;

            let mut ctx = RenderContext::new(page_dir.clone(), opts.dry_run, opts.should_overwrite);
            let renderer = ReadmeRenderer::new(&mut ctx);
            let markdown = renderer.render_page(page);

            write_file(&page_dir.join("README.md"), markdown.as_bytes(), opts)?;
            page_idx += 1;
        }
    }

    Ok(())
}

fn generate_section_readme(section_name: &str, pages: &[(String, String)]) -> String {
    let mut out = format!("# {}\n\n", section_name);
    for (title, dir_name) in pages {
        out.push_str(&format!("[{}]({})\n\n", title, dir_name));
    }
    out
}

fn ensure_dir(path: &Path, opts: &ConvertOptions) -> io::Result<()> {
    if opts.dry_run {
        info!("[dry-run] Would create directory: {:?}", path);
        return Ok(());
    }
    if path.exists() {
        if !opts.should_overwrite {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Directory already exists: {:?}", path),
            ));
        }
    } else {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn write_file(path: &Path, data: &[u8], opts: &ConvertOptions) -> io::Result<()> {
    if opts.dry_run {
        info!("[dry-run] Would write file: {:?} ({} bytes)", path, data.len());
        return Ok(());
    }
    if path.exists() && !opts.should_overwrite {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("File already exists: {:?}", path),
        ));
    }
    fs::write(path, data)
}
