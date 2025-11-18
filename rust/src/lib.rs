//! # HTML Minifier FFI Library
//!
//! A high-performance HTML, CSS, and JavaScript minifier built in Rust and exposed via FFI
//! for use in PHP and other languages.

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]

mod config;
pub mod constants;
mod ffi;
pub mod html;
mod minifiers;
mod token;
mod tokenizer;

pub use config::MinifierOptions;
pub use ffi::{minifier_clear_error, minifier_get_last_error, MinifierError};
pub use html::{minify_html_tokens, minify_html_with_options};
pub use minifiers::{minify_css, minify_javascript};
