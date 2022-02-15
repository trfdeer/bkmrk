use crate::{site_metadata::SiteMetadata, utils};
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

#[derive(Debug, Default, Clone)]
pub struct Bookmark {
    pub id: String,
    pub link: String,
    pub added_at: i64,
    pub last_modified: i64,
    pub metadata: SiteMetadata,
    pub tags: TagList,
}

#[cfg(feature = "tables")]
impl Tabled for Bookmark {
    const LENGTH: usize = 4;

    fn fields(&self) -> Vec<String> {
        vec![
            unescape::unescape(&self.metadata.title).unwrap_or(self.metadata.title.to_owned()),
            self.link.to_owned(),
            unescape::unescape(&self.metadata.description.to_owned().unwrap_or_default())
                .unwrap_or(self.metadata.description.to_owned().unwrap_or_default()),
            self.tags.to_string(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "Title".into(),
            "Link".into(),
            "Description".into(),
            "Tags".into(),
        ]
    }
}

impl Bookmark {
    pub fn format(&self, format_string: &str) -> String {
        let result = String::from(format_string);

        let result = result.replace("%n", &unescape::unescape(&self.metadata.title).unwrap());
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
            &unescape::unescape(&self.metadata.description.to_owned().unwrap_or_default()).unwrap(),
        );

        result
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} [{}]", self.metadata.title, self.link, self.tags,)
    }
}
