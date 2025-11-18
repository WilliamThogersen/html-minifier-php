//! MinifierContext for tracking HTML minification state

use crate::config::MinifierOptions;

pub struct MinifierContext {
    pub in_pre_tag: bool,
    pub in_script_tag: bool,
    pub in_style_tag: bool,
    pub current_tag: String,
    pub options: MinifierOptions,
}

impl MinifierContext {
    pub fn new(options: MinifierOptions) -> Self {
        Self {
            in_pre_tag: false,
            in_script_tag: false,
            in_style_tag: false,
            current_tag: String::new(),
            options,
        }
    }

    pub fn update_for_open_tag(&mut self, tag_name: &str) {
        self.current_tag.clear();
        self.current_tag.push_str(tag_name);
        self.current_tag.make_ascii_lowercase();

        self.in_pre_tag = matches!(self.current_tag.as_str(), "pre" | "code" | "textarea");
        self.in_script_tag = self.current_tag == "script";
        self.in_style_tag = self.current_tag == "style";
    }

    pub fn update_for_close_tag(&mut self, tag_name: &str) {
        // Use case-insensitive comparison to avoid allocation
        let tag_lower = tag_name.to_ascii_lowercase();
        if matches!(tag_lower.as_str(), "pre" | "code" | "textarea") {
            self.in_pre_tag = false;
        }
        if tag_lower == "script" {
            self.in_script_tag = false;
        }
        if tag_lower == "style" {
            self.in_style_tag = false;
        }
    }
}
