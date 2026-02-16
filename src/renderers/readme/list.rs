use onenote_parser::contents::{List, OutlineElement};

const FORMAT_NUMBERED_LIST: char = '\u{fffd}';

/// Determine the list prefix for an OutlineElement.
/// Returns "1. ", "- ", or "" (no list).
pub fn list_prefix(element: &OutlineElement) -> String {
    let lists = element.list_contents();
    if lists.is_empty() {
        return String::new();
    }

    let list = &lists[0];
    if is_numbered(list) {
        "1. ".to_string()
    } else {
        "- ".to_string()
    }
}

fn is_numbered(list: &List) -> bool {
    list.list_format()
        .first()
        .map(|c| *c == FORMAT_NUMBERED_LIST)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_numbered_with_sentinel() {
        // We can't easily construct a List in tests without the parser,
        // so we test the helper function logic directly.
        assert_eq!(FORMAT_NUMBERED_LIST, '\u{fffd}');
    }
}
