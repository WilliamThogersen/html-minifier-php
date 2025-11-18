//! CSS minification utilities

#[inline]
fn handle_css_string_literal(
    result: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    quote: char,
) {
    result.push(quote);

    while let Some(ch) = chars.next() {
        result.push(ch);
        if ch == quote {
            break;
        }
        if ch == '\\' {
            if let Some(escaped) = chars.next() {
                result.push(escaped);
            }
        }
    }
}

#[inline]
fn handle_css_comment(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
    if chars.peek() == Some(&'*') {
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

#[inline]
fn should_add_css_space(last_ch: char, next_char: Option<&char>) -> bool {
    // Don't add space after these characters
    if matches!(last_ch, '{' | '}' | ':' | ';' | ',' | '>' | '+' | '~' | '(' | '[') {
        return false;
    }

    // Don't add space before these characters
    if let Some(&next) = next_char {
        if matches!(next, '{' | '}' | ':' | ';' | ',' | ')' | ']') {
            return false;
        }
    }

    true
}

#[inline]
fn handle_css_whitespace(
    result: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    last_ch: char,
) {
    // Consume all whitespace
    while let Some(&next_ch) = chars.peek() {
        if next_ch.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }

    // Add a single space if needed
    if should_add_css_space(last_ch, chars.peek()) && last_ch != ' ' {
        result.push(' ');
    }
}

/// Minifies CSS code by removing comments and unnecessary whitespace.
///
/// This enhanced minifier handles:
/// - String literals (preserving content)
/// - Multi-line comments
/// - Smarter whitespace handling around selectors and properties
/// - Removes trailing semicolons before closing braces
///
/// # Arguments
///
/// * `css` - CSS source code as a string slice
///
/// # Returns
///
/// Minified CSS as a `String`
///
/// # Example
///
/// ```rust
/// use html_minifier_ffi::minify_css;
///
/// let css = "body {  color: red;  margin: 0;  }";
/// let minified = minify_css(css);
/// assert_eq!(minified, "body{color:red;margin:0}");
/// ```
#[inline]
pub fn minify_css(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut chars = css.chars().peekable();
    let mut last_ch = '\0';

    while let Some(ch) = chars.next() {
        match ch {
            '"' | '\'' => {
                handle_css_string_literal(&mut result, &mut chars, ch);
                last_ch = ch;
            }
            '/' => {
                if !handle_css_comment(&mut chars) {
                    result.push(ch);
                    last_ch = ch;
                }
            }
            c if c.is_whitespace() => {
                handle_css_whitespace(&mut result, &mut chars, last_ch);
                if let Some(&last) = result.as_bytes().last() {
                    last_ch = last as char;
                }
            }
            // Remove space before these characters and handle semicolon before }
            ':' | ',' | '{' | '>' | '+' | '~' => {
                // Remove trailing space
                if last_ch == ' ' {
                    result.pop();
                }
                result.push(ch);
                last_ch = ch;
            }
            ';' => {
                // Check if next non-whitespace char is '}'
                let mut temp_chars = chars.clone();
                while let Some(&next) = temp_chars.peek() {
                    if next.is_whitespace() {
                        temp_chars.next();
                    } else {
                        break;
                    }
                }

                // Only push semicolon if not followed by '}'
                if temp_chars.peek() != Some(&'}') {
                    if last_ch == ' ' {
                        result.pop();
                    }
                    result.push(ch);
                    last_ch = ch;
                } else {
                    last_ch = ';'; // Track but don't push
                }
            }
            '}' => {
                // Remove trailing space or semicolon
                if last_ch == ' ' || last_ch == ';' {
                    if let Some(last_byte) = result.as_bytes().last() {
                        if *last_byte == b' ' || *last_byte == b';' {
                            result.pop();
                        }
                    }
                }
                result.push(ch);
                last_ch = ch;
            }
            _ => {
                result.push(ch);
                last_ch = ch;
            }
        }
    }

    // Remove final trailing space
    if last_ch == ' ' {
        result.pop();
    }

    // Remove leading whitespace in-place (avoid allocation)
    let leading_ws = result.chars().take_while(|c| c.is_whitespace()).count();
    if leading_ws > 0 {
        result.drain(..leading_ws);
    }

    result
}
