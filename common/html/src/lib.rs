use std::{
    collections::HashMap,
    borrow::Cow,
};
use serde_json;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn to_html_tag(tag: String, attr: HashMap<String, String>) -> String {
    let attrs = attr.into_iter()
        .map(|(a, v)| {
            format!("{}={}", a, serde_json::to_string(&v).expect(""))
        })
        .collect::<Vec<String>>()
        .join(" ");
    format!("<{} {}>", tag, attrs)
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
