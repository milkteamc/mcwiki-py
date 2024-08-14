use pyo3::prelude::*;
use regex::Regex;

#[pyclass]
#[derive(Clone)]
pub struct WikiText {
    content: String,
}

#[pyclass]
pub enum TextMeta {
    Link { name: String, attrs: Vec<String> },
}

impl std::fmt::Debug for TextMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Link { .. } => write!(f, "{}", self.__repr__()),
        }
    }
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

#[pymethods]
impl WikiText {
    #[new]
    pub fn new(content: String) -> Self {
        WikiText { content }
    }

    pub fn get_meta(&self) -> PyResult<Vec<TextMeta>> {
        match get_text_meta(self.content.to_owned()) {
            Ok(m) => Ok(m),
            Err(reason) => Err(pyo3::exceptions::PyRuntimeError::new_err(reason)),
        }
    }

    pub fn __repr__(&self) -> String {
        format!("WikiText(meta={:?})", self.get_meta().unwrap())
    }
}

pub fn get_text_meta(text: String) -> Result<Vec<TextMeta>, &'static str> {
    let links_re = Regex::new(r"\[\[(.+?)\]\]").unwrap();
    let mut meta_items: Vec<TextMeta> = vec![];

    let captures = links_re.captures_iter(text.as_str());

    for cap in captures {
        let t = cap.get(1);
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
            None => return Err(
                "failed to capture links meta (occurred in get_text_meta, 'for cap in captures')",
            ),
        }
    }

    Ok(meta_items)
}
