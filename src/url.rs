use std::collections::HashMap;
use std::iter::IntoIterator;

pub struct UrlParams {
    elements: HashMap<String, String>
}

impl UrlParams {
    pub fn new() -> Self {
        UrlParams {
            elements: HashMap::new()
        }
    }

    pub fn add_monad<T: ToString>(mut self, key: &str, element: impl IntoIterator<Item = T>) -> Self {
        if let Some(e) = element.into_iter().next() {
            self.elements.insert(key.to_owned(), e.to_string());
        }
        
        self
    }

    pub fn add<T: ToString>(self, key: &str, element: T) -> Self {
        self.add_monad(key, Some(element))
    }

    pub fn build(&self) -> String {
        format!("?{}", self.elements.iter().map(|(key, value)| format!("{key}={value}")).collect::<Vec<String>>().join("&"))
    }
}
