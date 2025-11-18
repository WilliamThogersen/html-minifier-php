use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::{minify_html_tokens, minify_html_with_options, minify_javascript, MinifierOptions};

// Library version - must match PHP wrapper version
const LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");

// =============================================================================
// FFI Error Handling
// =============================================================================
//
// IMPORTANT: Error state is stored in thread-local storage.
// In PHP-FPM or similar multi-request environments, ensure errors are
// checked immediately after each FFI call, as the same thread may handle
// multiple requests. All FFI functions clear errors at the start to prevent
// leakage between calls.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinifierError {
    Success = 0,
    NullPointer = 1,
    InvalidUtf8 = 2,
    InternalError = 3,
}

thread_local! {
    static LAST_ERROR: std::cell::Cell<MinifierError> = const { std::cell::Cell::new(MinifierError::Success) };
    static LAST_ERROR_MESSAGE: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

fn set_last_error(error: MinifierError) {
    LAST_ERROR.with(|e| e.set(error));
    // Clear message when setting error without message
    LAST_ERROR_MESSAGE.with(|msg| {
        msg.borrow_mut().clear();
    });
}

fn set_last_error_with_message(error: MinifierError, message: String) {
    LAST_ERROR.with(|e| e.set(error));
    LAST_ERROR_MESSAGE.with(|msg| {
        *msg.borrow_mut() = message;
    });
}

#[no_mangle]
pub extern "C" fn minifier_get_last_error() -> MinifierError {
    LAST_ERROR.with(|e| e.get())
}

#[no_mangle]
pub extern "C" fn minifier_get_last_error_message() -> *mut c_char {
    LAST_ERROR_MESSAGE.with(|msg| {
        let message = msg.borrow();
        if message.is_empty() {
            std::ptr::null_mut()
        } else {
            match CString::new(message.as_str()) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn minifier_clear_error() {
    set_last_error(MinifierError::Success);
    LAST_ERROR_MESSAGE.with(|msg| {
        msg.borrow_mut().clear();
    });
}

// =============================================================================
// FFI Options
// =============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CMinifierOptions {
    pub remove_comments: bool,
    pub collapse_whitespace: bool,
    pub remove_optional_tags: bool,
    pub remove_attribute_quotes: bool,
    pub collapse_boolean_attributes: bool,
    pub remove_default_attributes: bool,
    pub remove_empty_attributes: bool,
    pub minify_js: bool,
    pub minify_css: bool,
    pub preserve_conditional_comments: bool,
}

impl From<CMinifierOptions> for MinifierOptions {
    fn from(c_opts: CMinifierOptions) -> Self {
        MinifierOptions {
            remove_comments: c_opts.remove_comments,
            collapse_whitespace: c_opts.collapse_whitespace,
            remove_optional_tags: c_opts.remove_optional_tags,
            remove_attribute_quotes: c_opts.remove_attribute_quotes,
            collapse_boolean_attributes: c_opts.collapse_boolean_attributes,
            remove_default_attributes: c_opts.remove_default_attributes,
            remove_empty_attributes: c_opts.remove_empty_attributes,
            minify_js: c_opts.minify_js,
            minify_css: c_opts.minify_css,
            preserve_conditional_comments: c_opts.preserve_conditional_comments,
        }
    }
}

impl From<MinifierOptions> for CMinifierOptions {
    fn from(opts: MinifierOptions) -> Self {
        CMinifierOptions {
            remove_comments: opts.remove_comments,
            collapse_whitespace: opts.collapse_whitespace,
            remove_optional_tags: opts.remove_optional_tags,
            remove_attribute_quotes: opts.remove_attribute_quotes,
            collapse_boolean_attributes: opts.collapse_boolean_attributes,
            remove_default_attributes: opts.remove_default_attributes,
            remove_empty_attributes: opts.remove_empty_attributes,
            minify_js: opts.minify_js,
            minify_css: opts.minify_css,
            preserve_conditional_comments: opts.preserve_conditional_comments,
        }
    }
}

#[no_mangle]
pub extern "C" fn minifier_options_default() -> CMinifierOptions {
    MinifierOptions::default().into()
}

#[no_mangle]
pub extern "C" fn minifier_options_conservative() -> CMinifierOptions {
    MinifierOptions::conservative().into()
}

// =============================================================================
// FFI Helper Functions
// =============================================================================

/// Validates and converts a C string pointer to a Rust string slice
/// Returns the string slice on success, or sets error and returns None
unsafe fn validate_and_convert_input<'a>(ptr: *const c_char, input_type: &str) -> Option<&'a str> {
    if ptr.is_null() {
        set_last_error_with_message(
            MinifierError::NullPointer,
            format!("{} pointer is null", input_type),
        );
        return None;
    }

    match CStr::from_ptr(ptr).to_str() {
        Ok(s) => Some(s),
        Err(e) => {
            set_last_error_with_message(
                MinifierError::InvalidUtf8,
                format!("Invalid UTF-8 in {}: {}", input_type, e),
            );
            None
        }
    }
}

/// Converts a Rust String to a C string pointer
/// Returns the pointer on success, or sets error and returns null
fn convert_output(output: String) -> *mut c_char {
    match CString::new(output) {
        Ok(c_string) => c_string.into_raw(),
        Err(e) => {
            set_last_error_with_message(
                MinifierError::InternalError,
                format!("Failed to create output string: {}", e),
            );
            std::ptr::null_mut()
        }
    }
}

// =============================================================================
// FFI Interface
// =============================================================================

/// Minifies HTML content from a C string pointer with default options
/// Returns a pointer to the minified string, or null on error
/// Caller must free the returned pointer using free_string()
///
/// # Safety
///
/// The caller must ensure that:
/// - `html_ptr` is either null or points to a valid, null-terminated C string
/// - The C string is valid UTF-8
/// - The pointer remains valid for the duration of this call
/// - The returned pointer must be freed using `free_string()`
///
/// # Error Handling
///
/// On error, returns null and sets the last error which can be retrieved using:
/// - `minifier_get_last_error()` - returns error code
/// - `minifier_get_last_error_message()` - returns error message (must be freed)
#[no_mangle]
pub unsafe extern "C" fn minify_html_string(html_ptr: *const c_char) -> *mut c_char {
    minifier_clear_error();

    let input = match validate_and_convert_input(html_ptr, "HTML") {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };

    let minified = minify_html_tokens(input);
    convert_output(minified)
}

/// Minifies HTML content from a C string pointer with custom options
/// Returns a pointer to the minified string, or null on error
/// Caller must free the returned pointer using free_string()
///
/// # Safety
///
/// The caller must ensure that:
/// - `html_ptr` is either null or points to a valid, null-terminated C string
/// - The C string is valid UTF-8
/// - The pointer remains valid for the duration of this call
/// - `options` is a valid CMinifierOptions struct
/// - The returned pointer must be freed using `free_string()`
///
/// # Error Handling
///
/// On error, returns null and sets the last error which can be retrieved using:
/// - `minifier_get_last_error()` - returns error code
/// - `minifier_get_last_error_message()` - returns error message (must be freed)
#[no_mangle]
pub unsafe extern "C" fn minify_html_string_with_options(
    html_ptr: *const c_char,
    options: CMinifierOptions,
) -> *mut c_char {
    minifier_clear_error();

    let input = match validate_and_convert_input(html_ptr, "HTML") {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };

    let rust_options: MinifierOptions = options.into();
    let minified = minify_html_with_options(input, &rust_options);
    convert_output(minified)
}

/// Minifies JavaScript content from a C string pointer
/// Returns a pointer to the minified string, or null on error
/// Caller must free the returned pointer using free_string()
///
/// # Safety
///
/// The caller must ensure that:
/// - `js_ptr` is either null or points to a valid, null-terminated C string
/// - The C string is valid UTF-8
/// - The pointer remains valid for the duration of this call
/// - The returned pointer must be freed using `free_string()`
///
/// # Error Handling
///
/// On error, returns null and sets the last error which can be retrieved using:
/// - `minifier_get_last_error()` - returns error code
/// - `minifier_get_last_error_message()` - returns error message (must be freed)
#[no_mangle]
pub unsafe extern "C" fn minify_javascript_string(js_ptr: *const c_char) -> *mut c_char {
    minifier_clear_error();

    let input = match validate_and_convert_input(js_ptr, "JavaScript") {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };

    let minified = minify_javascript(input);
    convert_output(minified)
}

/// Frees a string allocated by the minifier
/// Safe to call with null pointers
///
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` is either null or was previously returned by one of the minifier functions
/// - `ptr` has not been freed before
/// - `ptr` will not be used after this call
#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}

/// Get the library version string
/// Caller must free the returned pointer using free_string()
#[no_mangle]
pub extern "C" fn minifier_get_version() -> *mut c_char {
    match CString::new(LIBRARY_VERSION) {
        Ok(version) => version.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
