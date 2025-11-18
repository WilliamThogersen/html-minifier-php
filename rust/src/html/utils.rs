//! Utility functions for HTML attribute and whitespace processing

use crate::config::MinifierOptions;
use crate::constants::{
    has_default_value, is_boolean_attribute, is_empty_removable, should_remove_quotes,
};
use crate::minifiers::minify_css;
use std::borrow::Cow;

/// Collapses consecutive whitespace into single spaces
pub fn append_collapsed_whitespace(result: &mut String, content: &str) {
    let mut prev_was_space = false;
    for ch in content.chars() {
        if ch.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }
}

/// Processes and minifies style attribute values
pub fn process_style_attribute(value: &str) -> String {
    // Use the proper CSS minifier instead of manual processing
    // This ensures consistent handling with <style> tags
    let minified = minify_css(value);

    // Remove trailing semicolon for inline styles (it's optional)
    minified.trim_end_matches(';').to_string()
}

/// Processes and normalizes class attribute values
pub fn process_class_attribute(value: &str) -> String {
    let mut class_result = String::with_capacity(value.len());
    let mut prev_space = false;

    for ch in value.chars() {
        if ch.is_whitespace() {
            if !prev_space && !class_result.is_empty() {
                class_result.push(' ');
                prev_space = true;
            }
        } else {
            class_result.push(ch);
            prev_space = false;
        }
    }

    class_result
}

/// Process attribute value, only allocating if transformation is needed
pub fn process_attribute_value_cow<'a>(key: &str, value: &'a str) -> Cow<'a, str> {
    match key {
        "style" => Cow::Owned(process_style_attribute(value)),
        "class" if value.contains("  ") => Cow::Owned(process_class_attribute(value)),
        _ => Cow::Borrowed(value),
    }
}

/// Extracts attribute value from raw string (removes quotes if present)
pub fn extract_attribute_value(raw_value: &str) -> &str {
    if raw_value.len() >= 2
        && ((raw_value.starts_with('"') && raw_value.ends_with('"'))
            || (raw_value.starts_with('\'') && raw_value.ends_with('\'')))
    {
        &raw_value[1..raw_value.len() - 1]
    } else {
        raw_value
    }
}

/// Determines if an attribute should be skipped during minification
pub fn should_skip_attribute(key: &str, value: &str, current_tag: &str) -> bool {
    if is_boolean_attribute(key) {
        return false;
    }

    if value.is_empty() {
        if is_empty_removable(key) || matches!(key, "type" | "value" | "alt" | "title") {
            return true;
        }
    }

    has_default_value(current_tag, key, value)
}

/// Appends attribute value to result, adding quotes if necessary
pub fn append_attribute_value(
    result: &mut String,
    key: &str,
    value: &str,
    options: &MinifierOptions,
) {
    // Use Cow to avoid allocation when no processing is needed
    let processed_value = process_attribute_value_cow(key, value);

    if options.remove_attribute_quotes && should_remove_quotes(&processed_value) {
        result.push_str(&processed_value);
    } else {
        result.push('"');
        result.push_str(&processed_value);
        result.push('"');
    }
}

/// Processes a single attribute and appends it to the result
pub fn process_attribute(
    result: &mut String,
    attr: &str,
    current_tag: &str,
    options: &MinifierOptions,
) {
    let clean_attr = attr.trim();
    if clean_attr.is_empty() {
        return;
    }

    if let Some((key_part, raw_value_part)) = clean_attr.split_once('=') {
        let key = key_part.trim().to_lowercase();
        let raw_value = raw_value_part.trim();
        let value = extract_attribute_value(raw_value);

        if options.collapse_boolean_attributes && is_boolean_attribute(&key) {
            result.push(' ');
            result.push_str(&key);
            return;
        }

        if options.remove_empty_attributes && value.is_empty() && is_empty_removable(&key) {
            return;
        }

        if options.remove_default_attributes && should_skip_attribute(&key, value, current_tag) {
            return;
        }

        result.push(' ');
        result.push_str(&key);
        result.push('=');
        append_attribute_value(result, &key, value, options);
    } else {
        let key = clean_attr.to_lowercase();
        if !(options.remove_empty_attributes && is_empty_removable(&key)) {
            result.push(' ');
            result.push_str(&key);
        }
    }
}

fn skip_following_whitespace(chars: &mut std::iter::Peekable<std::str::Chars>) {
    while let Some(&next_ch) = chars.peek() {
        if next_ch.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn handle_closing_angle_bracket(
    cleaned: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars>,
) {
    cleaned.push('>');
    skip_following_whitespace(chars);
}

fn handle_whitespace_in_cleanup(
    cleaned: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars>,
) {
    if let Some(&'<') = chars.peek() {
        return;
    }

    if !cleaned.ends_with(' ') {
        cleaned.push(' ');
    }

    skip_following_whitespace(chars);
}

fn handle_equals_sign(cleaned: &mut String, chars: &mut std::iter::Peekable<std::str::Chars>) {
    while cleaned.ends_with(' ') {
        cleaned.pop();
    }
    cleaned.push('=');

    while let Some(&next_ch) = chars.peek() {
        if next_ch == ' ' {
            chars.next();
        } else {
            break;
        }
    }
}

/// Cleans up HTML spacing in a final pass
pub fn cleanup_html_spacing(html: &str) -> String {
    let mut cleaned = String::with_capacity(html.len());
    let mut chars = html.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '>' => handle_closing_angle_bracket(&mut cleaned, &mut chars),
            ch if ch.is_whitespace() => handle_whitespace_in_cleanup(&mut cleaned, &mut chars),
            '=' => handle_equals_sign(&mut cleaned, &mut chars),
            _ => cleaned.push(ch),
        }
    }

    cleaned.trim().to_string()
}
