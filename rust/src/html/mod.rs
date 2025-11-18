//! HTML minification module

pub mod context;
pub mod processor;
pub mod utils;

// Re-export main functions for convenience
pub use processor::{minify_html_tokens, minify_html_with_options};
