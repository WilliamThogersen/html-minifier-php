//! Minifiers for CSS and JavaScript

pub mod css;
pub mod javascript;

// Re-export main functions for convenience
pub use css::minify_css;
pub use javascript::minify_javascript;
