pub use std::collections::HashMap;

pub struct Dom {}

#[derive(Debug, PartialEq)]
pub struct Element {
    tag: &'static str,
    attrs: HashMap<&'static str, &'static str>,
}
impl Element {
    pub fn new(tag: &'static str) -> Element {
        Self { tag, attrs: HashMap::new() }     
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    
    #[test]
    pub fn can_create_element() {
        let elem = Element::new("button");
        let elem_lit = Element { tag: "button", attrs: HashMap::new() };
        assert_eq!(elem, elem_lit);
    }
}
