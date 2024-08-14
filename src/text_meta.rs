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

#[pyfunction]
pub fn get_text_meta(text: String) -> PyResult<Vec<TextMeta>> {
    let links_re = Regex::new(r"\[\[.+\]\]").unwrap();
    let mut meta_items: Vec<TextMeta> = vec![];

    let captures = links_re.captures_iter(text.as_str());

    for cap in captures {
        let t = cap.get(0);
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
            None => {
                return Err(pyo3::exceptions::PyRuntimeError::new_err(
                    "failed to capture links meta (occurred in get_text_meta, 'for cap in captures')",
                ))
            }
        }
    }

    Ok(meta_items)
}
