use html_minifier_ffi::constants::*;

#[test]
fn test_singleton_elements() {
    assert!(is_singleton_element("br"));
    assert!(is_singleton_element("img"));
    assert!(is_singleton_element("input"));
    assert!(!is_singleton_element("div"));
}

#[test]
fn test_boolean_attributes() {
    assert!(is_boolean_attribute("checked"));
    assert!(is_boolean_attribute("disabled"));
    assert!(is_boolean_attribute("readonly"));
    assert!(!is_boolean_attribute("id"));
}

#[test]
fn test_should_remove_quotes() {
    assert!(should_remove_quotes("simple"));
    assert!(should_remove_quotes("with-dash"));
    assert!(should_remove_quotes("with_underscore"));
    assert!(!should_remove_quotes("with space"));
    assert!(!should_remove_quotes("with=equals"));
    assert!(!should_remove_quotes(""));
}
