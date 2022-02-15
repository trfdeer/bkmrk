use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Default)]
pub struct Element {
    pub name: Tags,
    pub attributes: Option<HashMap<String, String>>,
    pub inner_text: Option<String>,
    pub children: Option<Vec<Element>>,
    pub text: Option<String>,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}|{:?}|{:?}|{:?}|{:?}",
            self.name, self.attributes, self.inner_text, self.children, self.text
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Tags {
    DOCTYPE,
    META,
    TITLE,
    H1,
    H3,
    DL,
    DT,
    DD,
    A,
    P,
    NONE,
}

impl From<&str> for Tags {
    fn from(tag: &str) -> Self {
        match tag.trim() {
            "DOCTYPE" => Self::DOCTYPE,
            "META" => Self::META,
            "TITLE" => Self::TITLE,
            "H1" => Self::H1,
            "H3" => Self::H3,
            "DL" => Self::DL,
            "DT" => Self::DT,
            "DD" => Self::DD,
            "A" => Self::A,
            "p" => Self::P,
            _ => unreachable!(),
        }
    }
}

impl Default for Tags {
    fn default() -> Self {
        Self::NONE
    }
}
