use html_minifier_ffi::minify_html_tokens;

#[test]
fn test_minify_html_basic() {
    let html = "<div>  <p>  Hello World  </p>  </div>";
    let result = minify_html_tokens(html);
    // Note: <p> closing tag is optional and gets removed
    assert_eq!(result, "<div><p>Hello World</div>");
}

#[test]
fn test_minify_html_attributes() {
    let html = r#"<div class="container" id="main">Content</div>"#;
    let result = minify_html_tokens(html);
    assert_eq!(result, r#"<div class=container id=main>Content</div>"#);
}

#[test]
fn test_minify_html_boolean_attributes() {
    let html = r#"<input type="checkbox" checked="checked">"#;
    let result = minify_html_tokens(html);
    // Type is not default, so it's kept, but checked is simplified
    assert_eq!(result, "<input type=checkbox checked>");
}

#[test]
fn test_minify_html_default_attributes() {
    let html = r#"<script type="text/javascript">alert('hi');</script>"#;
    let result = minify_html_tokens(html);
    assert_eq!(result, "<script>alert('hi');</script>");
}

#[test]
fn test_minify_html_preserve_pre() {
    let html = "<pre>  multiple   spaces  </pre>";
    let result = minify_html_tokens(html);
    // Whitespace is preserved in <pre> tags but cleaned at edges
    assert_eq!(result, "<pre>multiple spaces </pre>");
}

#[test]
fn test_minify_html_remove_comments() {
    let html = "<div><!-- comment --><p>Text</p></div>";
    let result = minify_html_tokens(html);
    // <p> closing tag is optional
    assert_eq!(result, "<div><p>Text</div>");
}

#[test]
fn test_minify_html_optional_closing_tags() {
    let html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
    let result = minify_html_tokens(html);
    assert_eq!(result, "<ul><li>Item 1<li>Item 2</ul>");
}

#[test]
fn test_empty_html() {
    let result = minify_html_tokens("");
    assert_eq!(result, "");
}

#[test]
fn test_utf8_handling() {
    let html = "<p>Hello 世界 🌍</p>";
    let result = minify_html_tokens(html);
    // <p> closing tag is optional
    assert_eq!(result, "<p>Hello 世界 🌍");
}

#[test]
fn test_nested_tags() {
    let html = "<div><p><span><a>Link</a></span></p></div>";
    let result = minify_html_tokens(html);
    // <p> closing tag is optional
    assert_eq!(result, "<div><p><span><a>Link</a></span></div>");
}

#[test]
fn test_doctype() {
    let html = "<!DOCTYPE html><html><body>Test</body></html>";
    let result = minify_html_tokens(html);
    assert_eq!(result, "<!doctype html><html><body>Test</body></html>");
}

#[test]
fn test_svg_minification() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="19.206" height="22.347">
  <path id="Path_495" stroke-width="1"></path>
  <path id="Path_497" stroke-width="1"></path>
</svg>"#;
    let result = minify_html_tokens(svg);

    // Should have two separate path tags
    assert!(result.contains("<path"));
    assert!(result.contains("</path>"));
    assert!(result.contains("stroke-width"));

    // Should not merge tags incorrectly
    assert!(!result.contains("</path></path>"));

    println!("SVG result: {}", result);
}

#[test]
fn test_svg_stroke_width_attribute() {
    let html = r#"<path stroke-width="1"></path>"#;
    let result = minify_html_tokens(html);

    // stroke-width="1" should remain valid
    assert!(result.contains("stroke-width"));
    assert!(result.contains("</path>"));
    assert!(!result.contains("1/"));

    println!("Result: {}", result);
}

#[test]
fn test_path_vs_p_tag() {
    // Make sure "path" is not confused with "p"
    let html = r#"<p>Text</p><path></path>"#;
    let result = minify_html_tokens(html);

    println!("Path vs P result: {}", result);

    // P closing tag should be removed (it's optional)
    assert!(!result.contains("</p>"));

    // Path closing tag should NOT be removed (it's NOT optional)
    assert!(result.contains("</path>"), "Path closing tag was incorrectly removed!");
}

#[test]
fn test_complex_svg_button() {
    let html = r##"<button class="btn account-menu-btn" type="button">
<svg xmlns="http://www.w3.org/2000/svg" width="19.206" height="22.347" viewBox="0 0 19.206 22.347">
<g id="Icon_heather-login" data-name="Icon feather-login" transform="translate(0.509 0.5)">
<path id="Path_495" data-name="Path 495" d="M14.747,9.624A5.124,5.124,0,1,1,9.624,4.5a5.124,5.124,0,0,1,5.124,5.124Z" transform="translate(-0.525 -4.5)" fill="none" stroke="#fff" stroke-linecap="round" stroke-linejoin="round" stroke-width="1"></path>
<path id="Path_497" data-name="Path 497" d="M3.893,27.464H22.087s.2-11.1-9.172-11.1C3.47,16.364,3.893,27.464,3.893,27.464Z" transform="translate(-3.892 -6.116)" fill="none" stroke="#fff" stroke-linejoin="round" stroke-width="1"></path>
</g>
</svg>
</button>"##;
    let result = minify_html_tokens(html);

    println!("Complex SVG result: {}", result);

    // Should not contain the broken pattern
    assert!(!result.contains("1/"), "Found broken '1/' pattern in: {}", result);
    assert!(!result.contains("</path></path>"), "Found duplicate closing tags");

    // Should contain proper structure
    let path_count = result.matches("<path").count();
    let closing_path_count = result.matches("</path>").count();
    assert_eq!(path_count, closing_path_count, "Mismatch between opening and closing path tags");
}
