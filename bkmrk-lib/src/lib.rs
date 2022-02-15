use std::path::Path;

use bookmark::Bookmark;
use chrono::Utc;
use eyre::{Result, WrapErr};

use crate::db::Database;

pub mod bookmark;
mod db;
mod element;
mod netscape_parser;
pub mod site_metadata;
mod utils;

pub struct BkmrkMan {
    db: Database,
}

impl BkmrkMan {
    pub fn new() -> Self {
        let db_path = utils::files::get_db_path().unwrap();
        let db = Database::connect(&db_path).unwrap();
        Self { db }
    }

    pub fn import_bookmark_file(
        &self,
        file_path: &Path,
        append_folder_tags: bool,
    ) -> Result<(usize, usize)> {
        let bookmarks =
            netscape_parser::parse_str(&utils::files::read_file(file_path)?, append_folder_tags)
                .wrap_err("Failed to parse bookmark file")?;

        let (succeeded, failed) = self.add_bookmarks(&bookmarks)?;
        Ok((succeeded, failed))
    }

    pub fn read_bookmark_file(
        &self,
        file_path: &Path,
        append_folder_tags: bool,
    ) -> Result<Vec<Bookmark>> {
        netscape_parser::parse_str(&utils::files::read_file(file_path)?, append_folder_tags)
            .wrap_err("Failed to parse bookmark file")
    }

    pub fn get_bookmarks(&self, tags: &[String], domains: &[String]) -> Result<Vec<Bookmark>> {
        let items = match (tags.len(), domains.len()) {
            (0, 0) => self.db.get_all(),
            _ => self.db.get(tags, domains),
        };

        items.wrap_err("Failed to get bookmarks from database")
    }

    pub fn add_bookmark(&self, bookmark: &Bookmark) -> Result<()> {
        let mut bookmark = bookmark.to_owned();
        let time_now = Utc::now().timestamp();
        bookmark.added_at = time_now;
        bookmark.last_modified = time_now;

        self.db.add_one(&bookmark)
    }

    pub fn add_bookmarks(&self, bookmark: &[Bookmark]) -> Result<(usize, usize)> {
        self.db.add_many(bookmark)
    }

    pub fn delete_bookmarks(&self, bookmarks: &[Bookmark]) -> Result<(usize, usize)> {
        self.db.delete_many(bookmarks)
    }
    pub fn update_bookmark_name(&self, old: &Bookmark, updated_val: &str) -> Result<()> {
        self.db.update_name(old, updated_val)
    }
    pub fn update_bookmark_link(&self, old: &Bookmark, updated_val: &str) -> Result<()> {
        self.db.update_link(old, updated_val)
    }
    pub fn update_bookmark_descr(&self, old: &Bookmark, updated_val: &str) -> Result<()> {
        self.db.update_descr(old, updated_val)
    }
    pub fn update_bookmark_site_type(&self, old: &Bookmark, updated_val: &str) -> Result<()> {
        self.db.update_site_type(old, updated_val)
    }
    pub fn update_bookmark_image_url(&self, old: &Bookmark, updated_val: &str) -> Result<()> {
        self.db.update_image_url(old, updated_val)
    }
    pub fn update_bookmark_tags(
        &self,
        old: &Bookmark,
        updated_val: &[String],
    ) -> Result<(usize, usize)> {
        self.db.update_tags(old, updated_val)
    }

    pub fn tag_rename(&self, tag_name: &str, new_tag_name: &str) -> Result<usize> {
        self.db.tag_rename(tag_name, new_tag_name)
    }

    pub fn tag_delete(&self, tag_name: &str) -> Result<usize> {
        self.db.tag_delete(tag_name)
    }
}
