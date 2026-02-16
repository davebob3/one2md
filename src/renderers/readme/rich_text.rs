use onenote_parser::contents::{ParagraphStyling, RichText};

const FORMAT_NUMBERED_LIST: char = '\u{fffd}';

/// Render RichText to Markdown.
pub fn render_rich_text(text: &RichText, in_table: bool) -> String {
    let raw = text.text();
    if raw.is_empty() {
        return String::new();
    }

    let paragraph_style = text.paragraph_style();

    // Check for heading via style_id
    let heading_prefix = heading_prefix_from_style(paragraph_style);

    let indices = text.text_run_indices();
    let run_styles = text.text_run_formatting();

    let mut result = String::new();

    if indices.is_empty() {
        // No text runs — apply paragraph style to entire text
        let (start_fmt, end_fmt) = formatting_markers(paragraph_style, None);
        result.push_str(&start_fmt);
        result.push_str(&sanitize_text(raw, in_table));
        result.push_str(&end_fmt);
    } else {
        // Split text by run indices and apply per-run formatting
        let chars: Vec<char> = raw.chars().collect();
        let mut pos: usize = 0;

        for (i, &end_idx) in indices.iter().enumerate() {
            let end = (end_idx as usize).min(chars.len());
            let segment: String = chars[pos..end].iter().collect();
            pos = end;

            let run_style = run_styles.get(i);
            let (start_fmt, end_fmt) = formatting_markers(paragraph_style, run_style);

            // Handle hyperlinks
            if let Some(style) = run_style {
                if style.hyperlink() {
                    // The text itself is the URL in OneNote hyperlinks
                    let display = sanitize_text(&segment, in_table);
                    result.push_str(&format!("[{}]({})", display, segment.trim()));
                    continue;
                }
            }

            result.push_str(&start_fmt);
            result.push_str(&sanitize_text(&segment, in_table));
            result.push_str(&end_fmt);
        }

        // Remainder after last index
        if pos < chars.len() {
            let remainder: String = chars[pos..].iter().collect();
            let run_style = run_styles.last();
            let (start_fmt, end_fmt) = formatting_markers(paragraph_style, run_style);
            result.push_str(&start_fmt);
            result.push_str(&sanitize_text(&remainder, in_table));
            result.push_str(&end_fmt);
        }
    }

    if let Some(prefix) = heading_prefix {
        format!("{}{}", prefix, result)
    } else {
        result
    }
}

/// Determine the Markdown heading prefix from the paragraph style_id.
fn heading_prefix_from_style(style: &ParagraphStyling) -> Option<String> {
    match style.style_id() {
        Some("h1") => Some("# ".to_string()),
        Some("h2") => Some("## ".to_string()),
        Some("h3") => Some("### ".to_string()),
        Some("h4") => Some("#### ".to_string()),
        Some("h5") => Some("##### ".to_string()),
        Some("h6") => Some("###### ".to_string()),
        Some("PageTitle") | Some("PageDateTime") => None,
        _ => None,
    }
}

/// Build start/end formatting markers from paragraph and optional run styles.
fn formatting_markers(
    paragraph_style: &ParagraphStyling,
    run_style: Option<&ParagraphStyling>,
) -> (String, String) {
    let mut start = String::new();
    let mut end = String::new();

    let bold = paragraph_style.bold() || run_style.map_or(false, |s| s.bold());
    let italic = paragraph_style.italic() || run_style.map_or(false, |s| s.italic());
    let strikethrough =
        paragraph_style.strikethrough() || run_style.map_or(false, |s| s.strikethrough());
    let underline = paragraph_style.underline() || run_style.map_or(false, |s| s.underline());

    if bold {
        start.push_str("**");
        end.insert_str(0, "**");
    }
    if italic {
        start.push('*');
        end.insert(0, '*');
    }
    if strikethrough {
        start.push_str("~~");
        end.insert_str(0, "~~");
    }
    if underline {
        start.push_str("<u>");
        end.insert_str(0, "</u>");
    }

    (start, end)
}

/// Sanitize text for Markdown output.
fn sanitize_text(text: &str, in_table: bool) -> String {
    let mut s = text.to_string();
    // Remove the list numbering sentinel character if present
    s = s.replace(FORMAT_NUMBERED_LIST, "");
    if in_table {
        // In tables, newlines become spaces and pipes are escaped
        s = s.replace('\n', " ").replace('|', "\\|");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_text_basic() {
        assert_eq!(sanitize_text("hello", false), "hello");
    }

    #[test]
    fn test_sanitize_text_in_table() {
        assert_eq!(sanitize_text("a|b\nc", true), "a\\|b c");
    }

    #[test]
    fn test_sanitize_removes_fffd() {
        let text = format!("{}1. item", FORMAT_NUMBERED_LIST);
        assert_eq!(sanitize_text(&text, false), "1. item");
    }
}
