use pyo3::prelude::*;
use regex::Regex;

#[pyclass]
pub enum TextMeta {
    Link { name: String, attrs: Vec<String> },
}

#[pymethods]
impl TextMeta {
    fn __repr__(&self) -> String {
        match self {
            Self::Link { name, attrs } => {
                format!("TextMeta::Link(name={:?}, attrs={:?})", name, attrs)
            }
        }
    }
}

pub fn parse_text(text: String) -> Vec<TextMeta> {
    let links_re = Regex::new(r"\[\[.+\]\]").unwrap();
    let mut meta_items: Vec<TextMeta> = vec![];

    links_re.captures_iter(text.as_str()).for_each(|c| {
        let t = c.get(0);
        match t {
            Some(m) => {
                let splits = m
                    .as_str()
                    .split('|')
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                meta_items.push(TextMeta::Link {
                    name: splits[splits.len() - 1].to_owned(),
                    attrs: splits[0..splits.len() - 1].into(),
                });
            }
            None => panic!("(links_re) unexpected: None"),
        }
    });

    meta_items
}
