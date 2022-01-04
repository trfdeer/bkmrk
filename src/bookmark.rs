use crate::{db::Database, utils};
use simple_error::{bail, SimpleError};
use std::fmt::Display;
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

#[derive(Debug, Clone, Default, Tabled)]
pub struct Bookmark {
    #[header(hidden = true)]
    pub id: String,
    pub name: String,
    pub link: String,
    #[header(hidden = true)]
    pub added_at: i64,
    #[header(hidden = true)]
    pub last_modified: i64,
    pub tags: TagList,
    pub description: String,
}

impl Bookmark {
    pub fn format(&self, format_string: &str) -> String {
        let result = String::from(format_string);

        let result = result.replace("%n", &self.name);
        let result = result.replace("%l", &self.link);
        let result = result.replace("%a", &utils::get_date_string(self.added_at));
        let result = result.replace("%m", &utils::get_date_string(self.last_modified));
        let result = result.replace("%t", &self.tags.0.join(", "));
        let result = result.replace("%d", self.description.as_str());

        result
    }

    pub fn get_matching_bookmarks(
        tags: &[String],
        domains: &[String],
    ) -> Result<Vec<Bookmark>, SimpleError> {
        let db_path = utils::files::get_db_path().unwrap();
        let db = Database::connect(&db_path).unwrap();

        let items = match (tags.len(), domains.len()) {
            (0, 0) => db.get_all_bookmarks(),
            _ => db.get_bookmarks(tags, domains),
        };

        match items {
            Ok(i) => Ok(i),
            Err(e) => bail!("ERROR: Failed to fetch bookmarks from database\n{}", e),
        }
    }

    pub fn get_table(bookmarks: &[Bookmark]) -> String {
        fn get_width(pct: f32) -> usize {
            let width = terminal_size::terminal_size().unwrap().0 .0 as usize;
            ((width as f32) * pct).round() as usize - 5
        }
        Table::new(bookmarks)
            .with(Style::PSEUDO)
            .with(
                Modify::new(Column(0..))
                    .with(Alignment::Horizontal(AlignmentHorizontal::Left))
                    .with(Alignment::Vertical(AlignmentVertical::Top)),
            )
            .with(Modify::new(Column(0..1)).with(MaxWidth::wrapping(get_width(0.2))))
            .with(Modify::new(Column(1..2)).with(MaxWidth::wrapping(get_width(0.4))))
            .with(Modify::new(Column(2..3)).with(MaxWidth::wrapping(get_width(0.2))))
            .with(Modify::new(Column(3..4)).with(MaxWidth::truncating(get_width(0.2), "...")))
            .to_string()
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} [{}]", self.name, self.link, self.tags,)
    }
}
