use html_minifier_ffi::minify_javascript;

#[test]
fn test_minify_javascript_basic() {
    let js = "function test() {  return 42;  }";
    let result = minify_javascript(js);
    assert_eq!(result, "function test(){return 42;}");
}

#[test]
fn test_minify_javascript_comments() {
    let js = "// comment\nvar x = 5; /* block */";
    let result = minify_javascript(js);
    assert_eq!(result, "var x=5;");
}

#[test]
fn test_minify_javascript_strings() {
    let js = r#"var s = "hello world"; var t = 'test';"#;
    let result = minify_javascript(js);
    assert_eq!(result, r#"var s="hello world";var t='test';"#);
}

#[test]
fn test_minify_javascript_division_vs_regex() {
    // Division operator - should preserve
    let js = "let result = a / b;";
    let result = minify_javascript(js);
    assert_eq!(result, "let result=a/b;");

    // Regex literal after assignment
    let js = "let pattern = /test/g;";
    let result = minify_javascript(js);
    assert_eq!(result, "let pattern=/test/g;");

    // Division after closing paren
    let js = "(a + b) / c";
    let result = minify_javascript(js);
    assert_eq!(result, "(a+b)/c");

    // Regex after return
    let js = "return /pattern/i;";
    let result = minify_javascript(js);
    assert_eq!(result, "return/pattern/i;");
}

#[test]
fn test_minify_javascript_template_literals() {
    let js = r#"const msg = `Hello ${name}`;"#;
    let result = minify_javascript(js);
    assert_eq!(result, r#"const msg=`Hello ${name}`;"#);
}
