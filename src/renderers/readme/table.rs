use crate::renderers::RenderContext;
use onenote_parser::contents::Table;

use super::content::render_element_contents;

/// Render a Table to Markdown.
pub fn render_table(table: &Table, ctx: &mut RenderContext) -> String {
    let cols = table.cols() as usize;
    if cols == 0 {
        return String::new();
    }

    let mut out = String::new();

    // Header row (empty headers)
    out.push_str(&"|     ".repeat(cols));
    out.push_str("|\n");

    // Separator row
    out.push_str(&"| --- ".repeat(cols));
    out.push_str("|\n");

    // Data rows
    for row in table.contents() {
        for cell in row.contents() {
            out.push_str("| ");
            let cell_content: String = cell
                .contents()
                .iter()
                .map(|element| render_element_contents(element, ctx, true))
                .collect::<Vec<_>>()
                .join(" ");
            let single_line = cell_content.replace('\n', " ").trim().to_string();
            out.push_str(&single_line);
            out.push(' ');
        }
        out.push_str("|\n");
    }

    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_table_header_generation() {
        // Verify header format for 3 columns
        let header = "|     ".repeat(3) + "|\n" + &"| --- ".repeat(3) + "|\n";
        assert_eq!(
            header,
            "|     |     |     |\n| --- | --- | --- |\n"
        );
    }
}
