use crate::token::Token;
use memchr::memchr;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    end: usize,
    bytes: &'a [u8],
    in_tag: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            end: input.len(),
            bytes: input.as_bytes(),
            in_tag: false,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.end {
            match self.bytes[self.position] {
                b' ' | b'\t' | b'\n' | b'\r' => self.position += 1,
                _ => break,
            }
        }
    }

    fn consume_until_bytes(&mut self, delimiter: &[u8]) -> &'a str {
        let start = self.position;
        let delimiter_len = delimiter.len();

        if delimiter_len == 1 {
            if let Some(pos) = memchr(delimiter[0], &self.bytes[self.position..]) {
                self.position += pos + 1;
                return &self.input[start..self.position - 1];
            } else {
                self.position = self.end;
                return &self.input[start..self.end];
            }
        }

        while self.position < self.end {
            if self.position + delimiter_len <= self.end
                && &self.bytes[self.position..self.position + delimiter_len] == delimiter
            {
                let result = &self.input[start..self.position];
                self.position += delimiter_len;
                return result;
            }
            self.position += 1;
        }

        &self.input[start..self.end]
    }

    fn consume_tag_name(&mut self) -> &'a str {
        let start = self.position;

        while self.position < self.end {
            match self.bytes[self.position] {
                b'>' | b'/' | b' ' | b'\t' | b'\n' | b'\r' => break,
                _ => self.position += 1,
            }
        }

        &self.input[start..self.position]
    }

    fn consume_until_byte(&mut self, byte: u8) -> &'a str {
        let start = self.position;

        // Use memchr for fast searching
        if let Some(pos) = memchr(byte, &self.bytes[self.position..]) {
            self.position += pos;
        } else {
            self.position = self.end;
        }

        &self.input[start..self.position]
    }

    fn consume_attribute_name(&mut self) -> bool {
        let mut has_equals = false;

        while self.position < self.end {
            match self.bytes[self.position] {
                b'=' => {
                    has_equals = true;
                    self.position += 1;
                    break;
                }
                b' ' | b'\t' | b'\n' | b'\r' | b'>' => break,
                _ => self.position += 1,
            }
        }

        has_equals
    }

    fn consume_quoted_value(&mut self, quote_char: u8) {
        self.position += 1;
        while self.position < self.end {
            if self.bytes[self.position] == quote_char {
                self.position += 1;
                break;
            }
            self.position += 1;
        }
    }

    fn consume_unquoted_value(&mut self) {
        while self.position < self.end {
            match self.bytes[self.position] {
                b' ' | b'\t' | b'\n' | b'\r' | b'>' => break,
                _ => self.position += 1,
            }
        }
    }

    fn consume_attribute(&mut self) -> Option<&'a str> {
        self.skip_whitespace();
        if self.position >= self.end || self.bytes[self.position] == b'>' {
            return None;
        }

        let start = self.position;
        let has_equals = self.consume_attribute_name();

        if has_equals {
            self.skip_whitespace();

            if self.position < self.end && matches!(self.bytes[self.position], b'"' | b'\'') {
                let quote_char = self.bytes[self.position];
                self.consume_quoted_value(quote_char);
            } else {
                self.consume_unquoted_value();
            }
        }

        if self.position > start {
            Some(&self.input[start..self.position])
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace();

        if self.position >= self.end {
            return None;
        }

        // Handle attributes if we're inside a tag
        if self.in_tag {
            if self.position < self.end && self.bytes[self.position] == b'>' {
                self.position += 1;
                self.in_tag = false;
                return Some(Token::TagOpenEnd);
            }

            if self.position + 1 < self.end
                && self.bytes[self.position] == b'/'
                && self.bytes[self.position + 1] == b'>'
            {
                self.position += 2;
                self.in_tag = false;
                return Some(Token::TagSelfClose);
            }

            if let Some(attr) = self.consume_attribute() {
                return Some(Token::Attribute(attr));
            }

            // If we can't parse an attribute, exit tag mode
            self.in_tag = false;
        }

        match self.bytes[self.position] {
            b'<' => self.parse_tag(),
            _ => self.parse_text_node(),
        }
    }

    fn parse_tag(&mut self) -> Option<Token<'a>> {
        self.position += 1;
        if self.position >= self.end {
            return None;
        }

        match self.bytes[self.position] {
            b'!' => self.parse_special_tag(),
            b'/' => self.parse_close_tag(),
            _ => self.parse_open_tag(),
        }
    }

    fn parse_special_tag(&mut self) -> Option<Token<'a>> {
        self.position += 1;

        if self.position + 2 < self.end && &self.bytes[self.position..self.position + 2] == b"--" {
            // Comment
            self.position += 2;
            let content = self.consume_until_bytes(b"-->");
            Some(Token::Comment(content))
        } else if self.position + 7 < self.end
            && &self.bytes[self.position..self.position + 7] == b"DOCTYPE"
        {
            // Doctype
            let start = self.position - 2;
            let _content = self.consume_until_byte(b'>');
            if self.position < self.end && self.bytes[self.position] == b'>' {
                self.position += 1;
            }
            Some(Token::Doctype(&self.input[start..self.position]))
        } else if self.position + 7 < self.end
            && &self.bytes[self.position..self.position + 7] == b"[CDATA["
        {
            // CDATA
            self.position += 7;
            let content = self.consume_until_bytes(b"]]>");
            Some(Token::Cdata(content))
        } else {
            // Other special content
            let start = self.position - 2;
            let _content = self.consume_until_byte(b'>');
            if self.position < self.end && self.bytes[self.position] == b'>' {
                self.position += 1;
            }
            Some(Token::Comment(&self.input[start..self.position]))
        }
    }

    fn parse_close_tag(&mut self) -> Option<Token<'a>> {
        self.position += 1;
        let tag_name = self.consume_until_byte(b'>');
        if self.position < self.end && self.bytes[self.position] == b'>' {
            self.position += 1;
        }
        Some(Token::TagClose(tag_name))
    }

    fn parse_open_tag(&mut self) -> Option<Token<'a>> {
        let tag_name = self.consume_tag_name();
        self.in_tag = true;
        Some(Token::TagOpenStart(tag_name))
    }

    fn parse_text_node(&mut self) -> Option<Token<'a>> {
        let start = self.position;

        // Use memchr to quickly find the next tag
        if let Some(pos) = memchr(b'<', &self.bytes[self.position..]) {
            self.position += pos;
        } else {
            self.position = self.end;
        }

        if self.position > start {
            Some(Token::TextNode(&self.input[start..self.position]))
        } else {
            None
        }
    }
}
