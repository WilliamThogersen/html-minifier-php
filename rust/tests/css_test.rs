use html_minifier_ffi::minify_css;

#[test]
fn test_minify_css_basic() {
    let css = "body {  color: red;  margin: 0;  }";
    let result = minify_css(css);
    // CSS minifier removes space after { and trailing semicolon
    assert_eq!(result, "body{color:red;margin:0}");
}

#[test]
fn test_minify_css_comments() {
    let css = "/* comment */ body { color: red; }";
    let result = minify_css(css);
    assert_eq!(result, "body{color:red}");
}

#[test]
fn test_minify_css_whitespace() {
    let css = ".class1,\n.class2 {\n  display: block;\n}";
    let result = minify_css(css);
    assert_eq!(result, ".class1,.class2{display:block}");
}
