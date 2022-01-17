use crate::utils;
use std::fmt::Display;
#[cfg(feature = "tables")]
use tabled::*;

#[derive(Default, Debug, Clone)]
pub struct TagList(pub Vec<String>);

impl Display for TagList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

impl From<Vec<String>> for TagList {
    fn from(items: Vec<String>) -> Self {
        TagList(items)
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "tables", derive(Tabled))]
pub struct Bookmark {
    #[cfg_attr(feature = "tables", header(hidden = true))]
    pub id: String,
    pub name: String,
    pub link: String,
    #[cfg_attr(feature = "tables", header(hidden = true))]
    pub added_at: i64,
    #[cfg_attr(feature = "tables", header(hidden = true))]
    pub last_modified: i64,
    pub tags: TagList,
    pub description: String,
}

impl Bookmark {
    pub fn format(&self, format_string: &str) -> String {
        let result = String::from(format_string);

        let result = result.replace("%n", &unescape::unescape(&self.name).unwrap());
        let result = result.replace("%l", &unescape::unescape(&self.link).unwrap());
        let result = result.replace(
            "%a",
            &unescape::unescape(&utils::get_date_string(self.added_at)).unwrap(),
        );
        let result = result.replace(
            "%m",
            &unescape::unescape(&utils::get_date_string(self.last_modified)).unwrap(),
        );
        let result = result.replace("%t", &unescape::unescape(&self.tags.0.join(", ")).unwrap());
        let result = result.replace(
            "%d",
            &unescape::unescape(self.description.as_str()).unwrap(),
        );

        result
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} [{}]", self.name, self.link, self.tags,)
    }
}
