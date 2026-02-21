pub mod content;
pub mod embedded;
pub mod image;
pub mod list;
pub mod outline;
pub mod rich_text;
pub mod table;

use crate::renderers::RenderContext;
use onenote_parser::contents::Outline;
use onenote_parser::page::{Page, PageContent};

pub struct ReadmeRenderer<'a> {
    ctx: &'a mut RenderContext,
}

impl<'a> ReadmeRenderer<'a> {
    pub fn new(ctx: &'a mut RenderContext) -> Self {
        Self { ctx }
    }

    /// Render a full page to Markdown.
    pub fn render_page(mut self, page: &Page) -> String {
        let mut out = String::new();

        // Title
        if let Some(title) = page.title_text() {
            out.push_str(&format!("# {}\n\n", title.trim()));
        }

        // Page contents
        for pc in page.contents() {
            let rendered = self.render_page_content(pc);
            if !rendered.is_empty() {
                out.push_str(&rendered);
                if !rendered.ends_with('\n') {
                    out.push('\n');
                }
                out.push('\n');
            }
        }

        out
    }

    fn render_page_content(&mut self, pc: &PageContent) -> String {
        match pc {
            PageContent::Outline(outline) => self.render_outline(outline),
            PageContent::Image(image) => image::render_image(image, self.ctx),
            PageContent::EmbeddedFile(file) => embedded::render_embedded_file(file, self.ctx),
            PageContent::Ink(_) => String::new(),
            PageContent::Unknown => String::new(),
        }
    }

    fn render_outline(&mut self, outline: &Outline) -> String {
        outline::render_outline(outline, self.ctx)
    }
}
