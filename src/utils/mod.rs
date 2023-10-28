
use web_sys::{Element};

pub type Position = (f64, f64);

pub trait AttributeExtractHelper {
    fn get_class_names(&self) -> Vec<String>;
    fn get_styles(&self) -> Vec<String>;
    fn parent_element_with_class(&self, class: String) -> Option<Element>;
}

fn extract_words_from_attribute(node: &Element, attribute: &str) -> Vec<String> {
    let binding = node.get_attribute(attribute).unwrap_or_default();
    let words = binding.split(' ').map(|c| c.trim().to_string()).collect::<Vec<String>>();
    words
}

impl AttributeExtractHelper for Element {
    fn get_class_names(&self) -> Vec<String> {
        extract_words_from_attribute(self, "class")
    }

    fn get_styles(&self) -> Vec<String> {
        extract_words_from_attribute(self, "style")
    }

    fn parent_element_with_class(&self, class: String) -> Option<Element> {
        let class_names = self.get_class_names();
        if class_names.contains(&class) {
            Some(self.clone())
        } else {
            self.parent_element().and_then(|node| node.parent_element_with_class(class))
        }
    }
}