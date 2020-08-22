pub mod dom;
pub mod io;

use std::{
    collections::HashMap,
    borrow::Cow,
};
use serde_json;

pub fn to_html_tag_start(tag: &str, attr: Vec<(String, String)>) -> String {
    let attrs = attr.into_iter()
        .map(|(a, v)| {
            format!("{}={}", a, serde_json::to_string(&v).expect(""))
        })
        .collect::<Vec<String>>()
        .join(" ");
    format!("<{} {}>", tag, attrs)
}

pub fn set_inner_text(tag: &str, text: &str, attr: Option<Vec<(String, String)>>) -> String {
    let mut html = String::new();
    if let Some(attr) = attr {
        html.push_str(to_html_tag_start(tag, attr).as_str());
    } else { 
        html.push_str(to_html_tag_start(tag, Vec::new()).as_str()) 
    }
    html.push_str(text);
    html.push_str(format!("</{}>", tag).as_str());
    html
}

pub fn html_escape(input: &str) -> Cow<str> {
    for (i, ch) in input.chars().enumerate() {
        if escape_ch(ch).is_some() {
            let mut esc_str = String::with_capacity(input.len());
            esc_str.push_str(&input[..i]);
            for ch in input[..i].chars() {
                match escape_ch(ch) {
                    Some(esc_ch) => esc_str.push_str(esc_ch),
                    None => esc_str.push(ch),
                };
            }
            return Cow::Owned(esc_str);
        }
    }
    Cow::Borrowed(input)
}

pub fn escape_ch(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#x27;"),
        _ => None,
    }
}

pub enum HtmlEscape {
    Amp,
    Lt,
    Gt,
    DoubleQuote,
    Backslash,
}

#[cfg(test)]
pub mod tests {

    use super::*;
    
    #[test]
    pub fn to_html_tag_start_works() {
        let tag = "a";
        let mut attrs = Vec::new();
        attrs.push(("href".to_string(), "http://chris.pecunies.com".to_string()));
        attrs.push(("class".to_string(), "link".to_string()));
        let to_html = dbg!(to_html_tag_start(tag, attrs));
        debug_assert_eq!(to_html, "<a href=\"http://chris.pecunies.com\" class=\"link\">");
    }

    #[test]
    pub fn set_inner_html_text_works() {
        let tag = "button";
        let mut attrs = Vec::new();
        attrs.push(("class".to_string(), "btn".to_string()));
        let to_html = dbg!(set_inner_text(tag, "submit", Some(attrs)));
        assert_eq!(to_html, "<button class=\"btn\">submit</button>");
    }

}
