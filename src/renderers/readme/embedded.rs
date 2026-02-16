use crate::notebook::name_utils::unique_file_name;
use crate::renderers::RenderContext;
use log::{info, warn};
use onenote_parser::contents::EmbeddedFile;
use std::fs;

/// Render an EmbeddedFile to Markdown and write file data to embedded/ directory.
pub fn render_embedded_file(file: &EmbeddedFile, ctx: &mut RenderContext) -> String {
    let filename = file.filename();
    let embedded_dir = ctx.embedded_dir();

    // Ensure embedded directory exists
    if !ctx.dry_run {
        if let Err(e) = fs::create_dir_all(&embedded_dir) {
            warn!("Failed to create embedded dir: {}", e);
            return String::new();
        }
    }

    let unique_name = unique_file_name(filename, &embedded_dir);

    if ctx.dry_run {
        info!(
            "[dry-run] Would write embedded file: embedded/{} ({} bytes)",
            unique_name,
            file.data().len()
        );
    } else if let Err(e) = fs::write(embedded_dir.join(&unique_name), file.data()) {
        warn!("Failed to write embedded file: {}", e);
        return String::new();
    }

    format!("[{}](embedded/{})", filename, unique_name)
}
