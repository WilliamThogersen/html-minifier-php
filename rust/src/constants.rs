//! HTML element and attribute constants using perfect hash functions for O(1) lookups

use phf::phf_set;

// =============================================================================
// HTML Element and Attribute Constants (O(1) Lookups)
// =============================================================================

pub static SINGLETON_ELEMENTS: phf::Set<&'static str> = phf_set! {
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
};

pub static CLOSE_OPTIONAL_ELEMENTS: phf::Set<&'static str> = phf_set! {
    "p", "dt", "dd", "li", "option", "thead", "th", "tbody", "tr", "td", "tfoot", "colgroup",
};

pub static BOOLEAN_ATTRIBUTES: phf::Set<&'static str> = phf_set! {
    "allowfullscreen",
    "async",
    "autofocus",
    "autoplay",
    "checked",
    "controls",
    "default",
    "defer",
    "disabled",
    "formnovalidate",
    "hidden",
    "inert",
    "ismap",
    "itemscope",
    "loop",
    "multiple",
    "muted",
    "nomodule",
    "novalidate",
    "open",
    "playsinline",
    "readonly",
    "required",
    "reversed",
    "selected",
    "typemustmatch",
};

pub static EMPTY_REMOVABLE_ATTRIBUTES: phf::Set<&'static str> = phf_set! {
    "id",
    "class",
    "style",
    "title",
    "action",
    "lang",
    "dir",
    "onfocus",
    "onblur",
    "onchange",
    "onclick",
    "ondblclick",
    "onmousedown",
    "onmouseup",
    "onmouseover",
    "onmousemove",
    "onmouseout",
    "onkeypress",
    "onkeydown",
    "onkeyup",
    "target",
};

// =============================================================================
// HTML Element Utilities
// =============================================================================

#[inline(always)]
pub fn is_singleton_element(tag: &str) -> bool {
    SINGLETON_ELEMENTS.contains(&tag)
}

#[inline(always)]
pub fn is_close_optional(tag: &str) -> bool {
    CLOSE_OPTIONAL_ELEMENTS.contains(&tag)
}

#[inline(always)]
pub fn is_boolean_attribute(attr: &str) -> bool {
    BOOLEAN_ATTRIBUTES.contains(&attr)
}

#[inline(always)]
pub fn is_empty_removable(attr: &str) -> bool {
    EMPTY_REMOVABLE_ATTRIBUTES.contains(&attr)
}

#[inline]
pub fn has_default_value(tag: &str, attr: &str, value: &str) -> bool {
    match (tag, attr, value) {
        ("script", "type", "text/javascript") => true,
        ("style", "type", "text/css") => true,
        ("style", "media", "all") => true,
        ("form", "method", "get") => true,
        ("form", "autocomplete", "on") => true,
        ("form", "enctype", "application/x-www-form-urlencoded") => true,
        ("input", "type", "text") => true,
        ("button", "type", "submit") => true,
        _ => false,
    }
}

#[inline]
pub fn should_remove_quotes(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    // Fast byte-level validation for ASCII-safe attribute values
    for &byte in value.as_bytes() {
        match byte {
            // Alphanumeric (ASCII only for performance)
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => continue,
            // Safe special characters
            b'-' | b'_' | b'.' | b':' | b'/' | b'#' | b'@' | b'%' | b'!' | b'*' | b'~' => continue,
            // Unsafe characters require quotes
            b' ' | b'\t' | b'\n' | b'\r' | b'"' | b'\'' | b'`' | b'=' | b'<' | b'>' | b'&'
            | b'?' | b'{' | b'}' | b'[' | b']' | b'(' | b')' | b';' | b',' | b'+' => {
                return false;
            }
            // Non-ASCII bytes require quotes for safety
            128..=255 => return false,
            // Any other character is unsafe
            _ => return false,
        }
    }

    true
}
