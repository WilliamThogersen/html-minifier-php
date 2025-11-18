//! HTML token processing and minification

use crate::config::MinifierOptions;
use crate::constants::{is_close_optional, is_singleton_element};
use crate::html::context::MinifierContext;
use crate::html::utils::{append_collapsed_whitespace, cleanup_html_spacing, process_attribute};
use crate::minifiers::{minify_css, minify_javascript};
use crate::token::Token;
use crate::tokenizer::Tokenizer;

fn handle_text_node(result: &mut String, content: &str, context: &MinifierContext) {
    if context.in_style_tag && context.options.minify_css {
        let minified_css = minify_css(content);
        result.push_str(&minified_css);
    } else if context.in_script_tag && context.options.minify_js {
        let minified_js = minify_javascript(content);
        result.push_str(&minified_js);
    } else if context.in_pre_tag || !context.options.collapse_whitespace {
        result.push_str(content);
    } else if context.options.collapse_whitespace {
        append_collapsed_whitespace(result, content);
    } else {
        result.push_str(content);
    }
}

pub fn handle_token(result: &mut String, token: Token, context: &mut MinifierContext) {
    match token {
        Token::Doctype(content) => {
            result.push_str(&content.to_lowercase());
        }
        Token::Comment(comment_text) => {
            // Check for conditional comments
            if context.options.preserve_conditional_comments && is_conditional_comment(comment_text)
            {
                result.push_str("<!--");
                result.push_str(comment_text);
                result.push_str("-->");
            } else if !context.options.remove_comments {
                result.push_str("<!--");
                result.push_str(comment_text);
                result.push_str("-->");
            }
            // Otherwise skip comments for minification
        }
        Token::Cdata(content) => {
            result.push_str("<![CDATA[");
            result.push_str(content);
            result.push_str("]]>");
        }
        Token::TagOpenStart(tag_name) => {
            context.update_for_open_tag(tag_name);
            result.push('<');
            result.push_str(&context.current_tag);
        }
        Token::Attribute(attr) => {
            process_attribute(result, attr, &context.current_tag, &context.options);
        }
        Token::TagOpenEnd => {
            result.push('>');
        }
        Token::TagSelfClose => {
            if is_singleton_element(&context.current_tag) {
                result.push('>');
            } else {
                result.push_str("/>");
            }
        }
        Token::TagClose(tag_name) => {
            let tag_lower = tag_name.to_ascii_lowercase();
            if !context.options.remove_optional_tags || !is_close_optional(&tag_lower) {
                result.push_str("</");
                result.push_str(&tag_lower);
                result.push('>');
            }
            context.update_for_close_tag(tag_name);
        }
        Token::TextNode(content) => {
            handle_text_node(result, content, context);
        }
    }
}

fn is_conditional_comment(comment: &str) -> bool {
    comment.starts_with("[if ") || comment.starts_with("[endif")
}

/// Minifies HTML content using tokenization with default options.
///
/// This function uses a custom tokenizer to parse HTML and intelligently minify it
/// while preserving semantic correctness. It handles:
///
/// - Whitespace collapsing (except in `<pre>`, `<code>`, `<textarea>`)
/// - Comment removal
/// - Boolean attribute simplification
/// - Removal of default attribute values
/// - Quote optimization for attribute values
/// - Optional closing tag removal
/// - Embedded CSS and JavaScript minification
///
/// # Arguments
///
/// * `html` - HTML source code as a string slice
///
/// # Returns
///
/// Minified HTML as a `String`
///
/// # Example
///
/// ```rust
/// use html_minifier_ffi::minify_html_tokens;
///
/// let html = r#"<div class="container">  <p>Hello World!</p>  </div>"#;
/// let minified = minify_html_tokens(html);
/// assert_eq!(minified, "<div class=container><p>Hello World!</div>");
/// ```
pub fn minify_html_tokens(html: &str) -> String {
    minify_html_with_options(html, &MinifierOptions::default())
}

/// Minifies HTML content using tokenization with custom options.
///
/// # Arguments
///
/// * `html` - HTML source code as a string slice
/// * `options` - Minification options
///
/// # Returns
///
/// Minified HTML as a `String`
///
/// # Example
///
/// ```rust
/// use html_minifier_ffi::{minify_html_with_options, MinifierOptions};
///
/// let html = r#"<div class="container">  <p>Hello World!</p>  </div>"#;
/// let options = MinifierOptions::conservative();
/// let minified = minify_html_with_options(html, &options);
/// ```
pub fn minify_html_with_options(html: &str, options: &MinifierOptions) -> String {
    // Minified HTML is typically 50-70% of original size
    // Using 60% (3/5) as a reasonable estimate to reduce reallocations
    let mut result = String::with_capacity(html.len() * 3 / 5);
    let mut tokenizer = Tokenizer::new(html);
    let mut context = MinifierContext::new(options.clone());

    while let Some(token) = tokenizer.next_token() {
        handle_token(&mut result, token, &mut context);
    }

    if options.collapse_whitespace {
        cleanup_html_spacing(&result)
    } else {
        result
    }
}
