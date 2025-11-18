#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    TextNode(&'a str),
    TagOpenStart(&'a str),
    Attribute(&'a str),
    TagOpenEnd,
    TagSelfClose,
    TagClose(&'a str),
    Comment(&'a str),
    Doctype(&'a str),
    Cdata(&'a str),
}
