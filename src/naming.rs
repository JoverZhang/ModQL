/// File naming and link generation for Markdown output.

/// Generate the file name for a module page.
///
/// Examples:
/// - `module_file_name("mycrate::utils")` -> `"module.mycrate.utils.md"`
/// - `module_file_name("mycrate::foo::bar")` -> `"module.mycrate.foo.bar.md"`
pub fn module_file_name(qualified_name: &str) -> String {
    let dotted = qualified_name.replace("::", ".");
    format!("module.{dotted}.md")
}

/// Generate the file name for an internal module page.
pub fn internal_module_file_name(qualified_name: &str) -> String {
    let dotted = qualified_name.replace("::", ".");
    format!("module.{dotted}.internal.md")
}

/// The crate root page file name.
pub fn crate_index_file() -> &'static str {
    "index.md"
}

/// Extract the first sentence from documentation text, for use as a synopsis
/// in listing pages.
pub fn synopsis(docs: &Option<String>) -> Option<String> {
    let docs = docs.as_ref()?;
    let trimmed = docs.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Find first sentence boundary: period followed by whitespace or end of string
    for (i, ch) in trimmed.char_indices() {
        if ch == '.' {
            let next_idx = i + ch.len_utf8();
            if next_idx >= trimmed.len() {
                // Period at end of string
                return Some(trimmed[..next_idx].to_string());
            }
            let next_char = trimmed[next_idx..].chars().next();
            if let Some(nc) = next_char {
                if nc.is_whitespace() {
                    return Some(trimmed[..next_idx].to_string());
                }
            }
        }
    }
    // No sentence boundary found; use first line
    let first_line = trimmed.lines().next().unwrap_or(trimmed);
    Some(first_line.to_string())
}

/// Extract the short name (last segment) from a qualified name.
/// e.g. "mycrate::foo::Bar" -> "Bar"
pub fn short_name(qualified_name: &str) -> &str {
    qualified_name.rsplit("::").next().unwrap_or(qualified_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_file_name() {
        assert_eq!(
            module_file_name("mycrate::utils"),
            "module.mycrate.utils.md"
        );
    }

    #[test]
    fn test_module_file_name_nested() {
        assert_eq!(
            module_file_name("mycrate::foo::bar"),
            "module.mycrate.foo.bar.md"
        );
    }

    #[test]
    fn test_internal_module_file_name() {
        assert_eq!(
            internal_module_file_name("mycrate::utils"),
            "module.mycrate.utils.internal.md"
        );
    }

    #[test]
    fn test_crate_index() {
        assert_eq!(crate_index_file(), "index.md");
    }

    #[test]
    fn test_synopsis_single_sentence() {
        let docs = Some("This is a test.".to_string());
        assert_eq!(synopsis(&docs), Some("This is a test.".to_string()));
    }

    #[test]
    fn test_synopsis_multiple_sentences() {
        let docs = Some("First sentence. Second sentence here.".to_string());
        assert_eq!(synopsis(&docs), Some("First sentence.".to_string()));
    }

    #[test]
    fn test_synopsis_no_period() {
        let docs = Some("No period here".to_string());
        assert_eq!(synopsis(&docs), Some("No period here".to_string()));
    }

    #[test]
    fn test_synopsis_empty() {
        let docs = Some("".to_string());
        assert_eq!(synopsis(&docs), None);
    }

    #[test]
    fn test_synopsis_none() {
        assert_eq!(synopsis(&None), None);
    }

    #[test]
    fn test_synopsis_period_in_middle_of_word() {
        let docs = Some("Use foo.bar for access.".to_string());
        assert_eq!(synopsis(&docs), Some("Use foo.bar for access.".to_string()));
    }

    #[test]
    fn test_short_name() {
        assert_eq!(short_name("mycrate::foo::Bar"), "Bar");
        assert_eq!(short_name("mycrate"), "mycrate");
        assert_eq!(short_name(""), "");
    }
}
