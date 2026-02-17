use crate::renderers::RenderContext;
use onenote_parser::contents::{Outline, OutlineElement, OutlineItem};

use super::content::render_element_contents;
use super::list;

/// Render a full Outline to Markdown.
pub fn render_outline(outline: &Outline, ctx: &mut RenderContext) -> String {
    let mut out = String::new();
    render_items(&outline.items(), 0, &mut out, ctx);
    out
}

fn render_items(
    items: &[OutlineItem],
    depth: u8,
    out: &mut String,
    ctx: &mut RenderContext,
) {
    for item in items {
        match item {
            OutlineItem::Element(element) => {
                render_element(element, depth, out, ctx);
            }
            OutlineItem::Group(group) => {
                let new_depth = depth + group.child_level();
                render_items(group.outlines(), new_depth, out, ctx);
            }
        }
    }
}

fn render_element(
    element: &OutlineElement,
    depth: u8,
    out: &mut String,
    ctx: &mut RenderContext,
) {
    let indent = "  ".repeat(depth as usize);
    let list_prefix = list::list_prefix(element);

    out.push_str(&indent);
    out.push_str(&list_prefix);

    let content = render_element_contents(element, ctx, false);
    out.push_str(&content);

    if !content.ends_with('\n') {
        out.push('\n');
    }

    // Render children
    if !element.children().is_empty() {
        let child_depth = depth + element.child_level();
        render_items(element.children(), child_depth, out, ctx);
    }
}
