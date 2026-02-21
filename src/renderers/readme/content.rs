use crate::renderers::RenderContext;
use onenote_parser::contents::{Content, OutlineElement};

use super::embedded;
use super::image;
use super::rich_text;
use super::table;

/// Render a single Content element to Markdown.
pub fn render_content(content: &Content, ctx: &mut RenderContext, in_table: bool) -> String {
    match content {
        Content::RichText(text) => rich_text::render_rich_text(text, in_table),
        Content::Table(tbl) => {
            if in_table {
                // Skip nested tables per spec
                String::new()
            } else {
                table::render_table(tbl, ctx)
            }
        }
        Content::Image(img) => image::render_image(img, ctx),
        Content::EmbeddedFile(file) => embedded::render_embedded_file(file, ctx),
        Content::Ink(_) => String::new(),
        Content::Unknown => String::new(),
    }
}

/// Render all contents of an OutlineElement to Markdown.
pub fn render_element_contents(
    element: &OutlineElement,
    ctx: &mut RenderContext,
    in_table: bool,
) -> String {
    element
        .contents()
        .iter()
        .map(|c| render_content(c, ctx, in_table))
        .collect::<Vec<_>>()
        .join("")
}
