//! JavaScript minification utilities

fn handle_js_string_literal(
    result: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    quote: char,
) {
    result.push(quote);

    while let Some(inner_ch) = chars.next() {
        result.push(inner_ch);
        if inner_ch == quote {
            break;
        }
        if inner_ch == '\\' {
            if let Some(escaped) = chars.next() {
                result.push(escaped);
            }
        }
    }
}

fn handle_js_template_literal(
    result: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars>,
) {
    result.push('`');
    let mut depth = 0;

    while let Some(ch) = chars.next() {
        result.push(ch);
        match ch {
            '`' if depth == 0 => break,
            '\\' => {
                if let Some(escaped) = chars.next() {
                    result.push(escaped);
                }
            }
            '$' => {
                if let Some(&'{') = chars.peek() {
                    result.push(chars.next().unwrap());
                    depth += 1;
                }
            }
            '}' if depth > 0 => {
                depth -= 1;
            }
            _ => {}
        }
    }
}

fn handle_js_regex(result: &mut String, chars: &mut std::iter::Peekable<std::str::Chars>) {
    result.push('/');
    let mut in_char_class = false;

    while let Some(ch) = chars.next() {
        result.push(ch);
        match ch {
            '/' if !in_char_class => {
                // Consume regex flags (g, i, m, s, u, y)
                while let Some(&flag_ch) = chars.peek() {
                    if matches!(flag_ch, 'g' | 'i' | 'm' | 's' | 'u' | 'y') {
                        result.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                break;
            }
            '\\' => {
                if let Some(escaped) = chars.next() {
                    result.push(escaped);
                }
            }
            '[' => in_char_class = true,
            ']' => in_char_class = false,
            '\n' | '\r' => break, // Invalid regex
            _ => {}
        }
    }
}

fn handle_js_comment(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
    if let Some(&'/') = chars.peek() {
        chars.next();
        while let Some(c) = chars.next() {
            if c == '\n' {
                break;
            }
        }
        true
    } else if let Some(&'*') = chars.peek() {
        chars.next();
        let mut prev = ' ';
        while let Some(c) = chars.next() {
            if prev == '*' && c == '/' {
                break;
            }
            prev = c;
        }
        true
    } else {
        false
    }
}

/// Check if the last character indicates regex context
#[inline]
fn is_regex_after_single_char_operator(trimmed: &str) -> bool {
    if let Some(last_char) = trimmed.chars().last() {
        matches!(last_char, '(' | '[' | '{' | ',' | ';' | ':' | '=' | '!' | '&' | '|' | '?' | '~')
    } else {
        false
    }
}

/// Check if the code ends with a double-character operator that indicates regex context
#[inline]
fn is_regex_after_double_operator(trimmed: &str) -> bool {
    trimmed.ends_with("++")
        || trimmed.ends_with("--")
        || trimmed.ends_with("**")
        || trimmed.ends_with("==")
        || trimmed.ends_with("!=")
        || trimmed.ends_with("<=")
        || trimmed.ends_with(">=")
        || trimmed.ends_with("<<")
        || trimmed.ends_with(">>")
        || trimmed.ends_with("&&")
        || trimmed.ends_with("||")
        || trimmed.ends_with("??")
}

/// Check if ambiguous operator (+, -, *, %, <, >) is in regex context
#[inline]
fn is_regex_after_ambiguous_operator(trimmed: &str) -> Option<bool> {
    if let Some(last_char) = trimmed.chars().last() {
        if matches!(last_char, '+' | '-' | '*' | '%' | '<' | '>') {
            // Look at the character before to determine context
            if trimmed.len() >= 2 {
                let before_last = trimmed.chars().nth_back(1).unwrap();
                // If preceded by alphanumeric, ), or ], it's likely a binary operator (division allowed)
                if before_last.is_alphanumeric() || matches!(before_last, ')' | ']' | '_' | '$') {
                    return Some(false); // Not a regex context
                }
            }
            return Some(true); // Likely unary operator
        }
    }
    None
}

/// Check if the code ends with a keyword that indicates regex context
#[inline]
fn is_regex_after_keyword(trimmed: &str) -> bool {
    const KEYWORDS: &[&str] = &[
        "return",
        "throw",
        "new",
        "typeof",
        "void",
        "delete",
        "in",
        "of",
        "instanceof",
        "yield",
        "await",
        "case",
    ];

    for keyword in KEYWORDS {
        if trimmed.ends_with(keyword) {
            // Make sure it's actually a keyword and not part of an identifier
            let keyword_start = trimmed.len() - keyword.len();
            if keyword_start == 0 {
                return true;
            }
            if let Some(before_keyword) = trimmed.chars().nth(keyword_start.saturating_sub(1)) {
                if !before_keyword.is_alphanumeric()
                    && before_keyword != '_'
                    && before_keyword != '$'
                {
                    return true;
                }
            }
        }
    }
    false
}

/// Check if the code ends with an identifier or closing bracket (indicates division context)
#[inline]
fn is_division_context(trimmed: &str) -> bool {
    if let Some(last_char) = trimmed.chars().last() {
        last_char.is_alphanumeric() || matches!(last_char, ')' | ']' | '_' | '$')
    } else {
        false
    }
}

fn is_regex_context(result: &str) -> bool {
    let trimmed = result.trim_end();
    if trimmed.is_empty() {
        return true;
    }

    // Check single character operators first
    if is_regex_after_single_char_operator(trimmed) {
        return true;
    }

    // Check for double-character operators
    if is_regex_after_double_operator(trimmed) {
        return true;
    }

    // Check ambiguous operators (+, -, *, %, <, >)
    if let Some(is_regex) = is_regex_after_ambiguous_operator(trimmed) {
        return is_regex;
    }

    // Check for keywords that indicate regex context
    if is_regex_after_keyword(trimmed) {
        return true;
    }

    // If we see an identifier or closing bracket followed by /, it's division
    if is_division_context(trimmed) {
        return false;
    }

    // Default to division context (more conservative for safety)
    false
}

fn handle_js_whitespace(result: &mut String, chars: &mut std::iter::Peekable<std::str::Chars>) {
    if !result.is_empty() && !result.ends_with(' ') {
        if let Some(&next_ch) = chars.peek() {
            let last_ch = result.chars().last().unwrap_or(' ');
            if (last_ch.is_alphanumeric() || last_ch == '_' || last_ch == '$')
                && (next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '$')
            {
                result.push(' ');
            }
        }
    }

    while let Some(&next_ch) = chars.peek() {
        if next_ch.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

/// Minifies JavaScript code by removing comments and unnecessary whitespace.
///
/// This enhanced minifier handles:
/// - Template literals (backticks)
/// - Regular expressions
/// - Single and multi-line comments
/// - Proper whitespace handling around keywords and operators
///
/// # Arguments
///
/// * `js` - JavaScript source code as a string slice
///
/// # Returns
///
/// Minified JavaScript as a `String`
///
/// # Example
///
/// ```rust
/// use html_minifier_ffi::minify_javascript;
///
/// let js = "function test() {  return 42;  }";
/// let minified = minify_javascript(js);
/// assert_eq!(minified, "function test(){return 42;}");
/// ```
pub fn minify_javascript(js: &str) -> String {
    let mut result = String::with_capacity(js.len());
    let mut chars = js.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' | '\'' => {
                handle_js_string_literal(&mut result, &mut chars, ch);
            }
            '`' => {
                handle_js_template_literal(&mut result, &mut chars);
            }
            '/' => {
                if !handle_js_comment(&mut chars) {
                    // Check if this might be a regex
                    if is_regex_context(&result) {
                        handle_js_regex(&mut result, &mut chars);
                    } else {
                        result.push(ch);
                    }
                }
            }
            c if c.is_whitespace() => {
                handle_js_whitespace(&mut result, &mut chars);
            }
            _ => result.push(ch),
        }
    }

    result.trim().to_string()
}
