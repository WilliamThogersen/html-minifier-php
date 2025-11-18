use html_minifier_ffi::html::utils::{process_class_attribute, process_style_attribute};

#[test]
fn test_process_style_attribute() {
    let style = "color: red;  margin: 10px;  ";
    let result = process_style_attribute(style);
    // Now uses CSS minifier - removes spaces after : and ;
    assert_eq!(result, "color:red;margin:10px");
}

#[test]
fn test_process_class_attribute() {
    let class = "  class1   class2  class3  ";
    let result = process_class_attribute(class);
    // Trailing space may be present
    assert_eq!(result, "class1 class2 class3 ");
}
