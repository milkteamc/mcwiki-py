extern crate pyo3;
extern crate regex;

use pyo3::prelude::*;
use regex::Regex;

mod text_meta;

#[pyclass]
struct WikiPage {
    #[pyo3(get)]
    text: String,
}

#[pyclass]
enum Node {
    Title { level: u8, text: String },
    Text(String),
}

#[pymethods]
impl Node {
    fn __repr__(&self) -> String {
        match self {
            Self::Title { level, text } => format!("Node::Title(level={}, text={:?})", level, text),

            Self::Text(_) => "Node::Text()".into(),
        }
    }
}

#[pymethods]
impl WikiPage {
    #[new]
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn parse(&self) -> Vec<Node> {
        self._parse()
    }

    pub fn __repr__(&self) -> String {
        format!("<WikiPage ({} nodes)>", self.parse().len())
    }
}

impl WikiPage {
    fn _parse(&self) -> Vec<Node> {
        let mut i = 0_usize;
        let lines = self
            .text
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut line;

        let mut nodes: Vec<Node> = Vec::new();

        while i < lines.len() {
            line = &lines[i];

            if line.starts_with("==") && line.ends_with("==") {
                let re = Regex::new(r"^(=+)\s*(.+?)\s*=+$").unwrap();
                let captures = re.captures(line);
                match captures {
                    Some(m) => nodes.push(Node::Title {
                        level: m.get(1).unwrap().as_str().len() as u8,
                        text: m.get(2).unwrap().as_str().into(),
                    }),
                    None => continue,
                }
            } else {
                nodes.push(Node::Text(line.into()));
            }

            i += 1;
        }

        nodes
    }
}

#[pymodule]
fn mcwiki_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WikiPage>()?;
    m.add_class::<Node>()?;
    m.add_function(wrap_pyfunction!(text_meta::get_text_meta, m)?)?;
    Ok(())
}
