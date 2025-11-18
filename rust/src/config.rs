//! Configuration options for HTML minification

/// Configuration options for HTML minification
#[derive(Debug, Clone)]
pub struct MinifierOptions {
    /// Remove HTML comments (default: true)
    pub remove_comments: bool,
    /// Collapse whitespace in text nodes (default: true)
    pub collapse_whitespace: bool,
    /// Remove optional closing tags like </p>, </li> (default: true)
    pub remove_optional_tags: bool,
    /// Remove quotes from attributes when safe (default: true)
    pub remove_attribute_quotes: bool,
    /// Remove boolean attribute values (default: true)
    pub collapse_boolean_attributes: bool,
    /// Remove attributes with default values (default: true)
    pub remove_default_attributes: bool,
    /// Remove empty attributes (default: true)
    pub remove_empty_attributes: bool,
    /// Minify inline JavaScript (default: true)
    pub minify_js: bool,
    /// Minify inline CSS (default: true)
    pub minify_css: bool,
    /// Preserve conditional comments (default: false)
    pub preserve_conditional_comments: bool,
}

impl Default for MinifierOptions {
    fn default() -> Self {
        Self {
            remove_comments: true,
            collapse_whitespace: true,
            remove_optional_tags: true,
            remove_attribute_quotes: true,
            collapse_boolean_attributes: true,
            remove_default_attributes: true,
            remove_empty_attributes: true,
            minify_js: true,
            minify_css: true,
            preserve_conditional_comments: false,
        }
    }
}

impl MinifierOptions {
    /// Create a new MinifierOptions with all features enabled
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a conservative MinifierOptions (safer but less aggressive)
    pub fn conservative() -> Self {
        Self {
            remove_comments: true,
            collapse_whitespace: true,
            remove_optional_tags: false,
            remove_attribute_quotes: false,
            collapse_boolean_attributes: true,
            remove_default_attributes: false,
            remove_empty_attributes: false,
            minify_js: true,
            minify_css: true,
            preserve_conditional_comments: true,
        }
    }
}
