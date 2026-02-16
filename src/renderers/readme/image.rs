use crate::notebook::name_utils::unique_file_name;
use crate::renderers::RenderContext;
use log::{info, warn};
use onenote_parser::contents::Image;
use std::fs;

/// Render an Image to Markdown and write image data to embedded/ directory.
pub fn render_image(image: &Image, ctx: &mut RenderContext) -> String {
    let data = match image.data() {
        Some(d) => d,
        None => {
            warn!("Image has no data");
            return String::new();
        }
    };

    let filename = determine_filename(image);
    let embedded_dir = ctx.embedded_dir();

    // Ensure embedded directory exists
    if !ctx.dry_run {
        if let Err(e) = fs::create_dir_all(&embedded_dir) {
            warn!("Failed to create embedded dir: {}", e);
            return String::new();
        }
    }

    let unique_name = unique_file_name(&filename, &embedded_dir);

    if ctx.dry_run {
        info!(
            "[dry-run] Would write image: embedded/{} ({} bytes)",
            unique_name,
            data.len()
        );
    } else if let Err(e) = fs::write(embedded_dir.join(&unique_name), data) {
        warn!("Failed to write image: {}", e);
        return String::new();
    }

    let alt = image.alt_text().unwrap_or("");
    format!("![{}](embedded/{})", alt, unique_name)
}

fn determine_filename(image: &Image) -> String {
    if let Some(name) = image.image_filename() {
        if !name.is_empty() {
            return name.to_string();
        }
    }
    // Generate a name from extension
    let ext = image.extension().unwrap_or("png");
    format!("image.{}", ext)
}
