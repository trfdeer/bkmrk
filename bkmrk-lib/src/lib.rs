use std::path::Path;

use bookmark::Bookmark;
use simple_error::{bail, SimpleError};

use crate::db::Database;

pub mod bookmark;
mod db;
mod netscape_bookmark_parser;
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

    pub fn parse_netscape_file(
        &self,
        file_path: &Path,
        append_folder_tags: bool,
    ) -> Result<Vec<Bookmark>, SimpleError> {
        netscape_bookmark_parser::parse_netscape_bookmark_file(file_path, append_folder_tags)
    }

    pub fn get_bookmarks(
        &self,
        tags: &[String],
        domains: &[String],
    ) -> Result<Vec<Bookmark>, SimpleError> {
        let items = match (tags.len(), domains.len()) {
            (0, 0) => self.db.get_all(),
            _ => self.db.get(tags, domains),
        };

        match items {
            Ok(i) => Ok(i),
            Err(e) => bail!("ERROR: Failed to fetch bookmarks from database\n{}", e),
        }
    }

    pub fn add_bookmark(&self, bookmark: &Bookmark) -> Result<(), SimpleError> {
        self.db.add_one(bookmark)
    }

    pub fn add_bookmarks(&self, bookmark: &[Bookmark]) -> Result<(usize, usize), SimpleError> {
        self.db.add_many(bookmark)
    }

    pub fn delete_bookmarks(&self, bookmarks: &[Bookmark]) -> Result<(usize, usize), SimpleError> {
        self.db.delete_many(bookmarks)
    }
    pub fn update_bookmark_name(
        &self,
        old: &Bookmark,
        updated_val: &str,
    ) -> Result<(), SimpleError> {
        self.db.update_name(old, updated_val)
    }
    pub fn update_bookmark_link(
        &self,
        old: &Bookmark,
        updated_val: &str,
    ) -> Result<(), SimpleError> {
        self.db.update_link(old, updated_val)
    }
    pub fn update_bookmark_descr(
        &self,
        old: &Bookmark,
        updated_val: &str,
    ) -> Result<(), SimpleError> {
        self.db.update_descr(old, updated_val)
    }
    pub fn update_bookmark_tags(
        &self,
        old: &Bookmark,
        updated_val: &[String],
    ) -> Result<(), SimpleError> {
        self.db.update_tags(old, updated_val)
    }

    pub fn tag_rename(&self, tag_name: &str, new_tag_name: &str) -> Result<usize, SimpleError> {
        self.db.tag_rename(tag_name, new_tag_name)
    }

    pub fn tag_delete(&self, tag_name: &str) -> Result<usize, SimpleError> {
        self.db.tag_delete(tag_name)
    }
}
