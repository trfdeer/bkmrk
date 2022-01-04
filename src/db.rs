use std::{collections::HashSet, fs::File, path::Path};

use chrono::Utc;
use itertools::Itertools;
use log::{error, info};
use nanoid::nanoid;
use regex::Regex;
use rusqlite::{params, Connection};
use simple_error::{bail, SimpleError};

use crate::bookmark::{Bookmark, TagList};

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

#[allow(dead_code)]
impl Database {
    pub fn create_tables(&self) -> Result<(), SimpleError> {
        let query = "
            CREATE TABLE IF NOT EXISTS `Bookmark`(
                id CHAR(6) PRIMARY KEY,
                name VARCHAR(300) NOT NULL,
                link VARCHAR(300) NOT NULL UNIQUE,
                added_at DATETIME NOT NULL,
                last_modified DATETIME NOT NULL,
                description TEXT
            );

            CREATE TABLE IF NOT EXISTS `Tag`(
                bookmark_id NOT NULL,
                tag VARCHAR(100),
                FOREIGN KEY (bookmark_id) REFERENCES `Bookmark`(id)
            );";
        match self.conn.execute_batch(query) {
            Ok(_) => info!("Created tables"),
            Err(e) => bail!("ERROR: Couldn't create tables: {}", e),
        }

        Ok(())
    }

    pub fn connect(path: &Path) -> Result<Self, SimpleError> {
        let mut new_db = false;
        if !path.exists() {
            new_db = true;
            match File::create(path) {
                Ok(_) => (),
                Err(e) => bail!(
                    "ERROR: Couldn't create database file at {}: {}",
                    path.display(),
                    e
                ),
            };
        }
        let db = match Connection::open(path) {
            Ok(conn) => Self { conn },
            Err(e) => bail!("ERROR: Couldn't connect to db: {}", e),
        };
        if new_db {
            match db.create_tables() {
                Ok(_) => Ok(db),
                Err(e) => bail!("ERROR: Couldn't create database tables: {}", e),
            }
        } else {
            Ok(db)
        }
    }

    pub fn add_bookmark(&self, bookmark: &Bookmark) -> Result<(), SimpleError> {
        let id = nanoid!(
            6,
            &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f']
        );
        match self.conn.execute(
        "INSERT INTO `Bookmark` (id, name, link, added_at, last_modified, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            id,
            Regex::new("[\"']").unwrap().replace_all(&bookmark.name.replace('\\', r"\\"), "''"),
            Regex::new("[\"']").unwrap().replace_all(&bookmark.link, "''"),
            bookmark.added_at,
            bookmark.last_modified,
            Regex::new("[\"']").unwrap().replace_all(&bookmark.description, "''"),
        ],
    ) {
        Ok(_) => (),
        Err(e) => bail!("ERROR: Failed to save bookmark to database: {}", e)
    };

        for tag in &bookmark.tags.0 {
            self.conn
                .execute(
                    "INSERT INTO `Tag` (bookmark_id, tag) VALUES (?1, ?2)",
                    params![id, Regex::new("[\"']").unwrap().replace_all(tag, "''"),],
                )
                .unwrap();
        }

        Ok(())
    }

    pub fn add_bookmarks(&self, bookmarks: &[Bookmark]) -> Result<(usize, usize), SimpleError> {
        let (mut succeeded, mut failed) = (0, 0);
        for bookmark in bookmarks {
            match self.add_bookmark(bookmark) {
                Ok(_) => {
                    info!("Added bookmark '{}'", bookmark.name);
                    succeeded += 1;
                }
                Err(e) => {
                    error!("Couldn't add \"{}\": {}", bookmark.name, e);
                    failed += 1;
                }
            };
        }

        Ok((succeeded, failed))
    }

    pub fn delete_bookmark(&self, bookmark_id: &str) -> Result<(), SimpleError> {
        match self.conn.execute(
            &format!("DELETE FROM `Tag` WHERE bookmark_id LIKE '{}'", bookmark_id),
            [],
        ) {
            Ok(_) => (),
            Err(e) => bail!("ERROR: Failed to delete related tags: {}", e),
        }

        match self.conn.execute(
            &format!("DELETE FROM `Bookmark` WHERE id LIKE '{}'", bookmark_id),
            [],
        ) {
            Ok(_) => (),
            Err(e) => bail!("ERROR: Failed to remove bookmark from database: {}", e),
        }

        Ok(())
    }

    pub fn delete_bookmarks(&self, bookmarks: &[Bookmark]) -> Result<(usize, usize), SimpleError> {
        let (mut succeeded, mut failed) = (0, 0);
        for bookmark in bookmarks {
            match self.delete_bookmark(&bookmark.id) {
                Ok(_) => {
                    info!("Deleted {}", bookmark.name);
                    succeeded += 1;
                }
                Err(e) => {
                    error!(
                        "ERROR: Failed to delete bookmark: \"{}\"\n{}",
                        bookmark.name, e
                    );
                    failed += 1;
                }
            }
        }
        Ok((succeeded, failed))
    }

    pub fn get_all_bookmarks(&self) -> Result<Vec<Bookmark>, SimpleError> {
        let mut select_statement = self.conn.prepare("SELECT * FROM `Bookmark`").unwrap();
        let matches: Vec<Bookmark> = select_statement
            .query_map([], |row| {
                let id: String = row.get_unwrap(0);
                let item = Bookmark {
                    id: row.get_unwrap(0),
                    name: row.get_unwrap(1),
                    link: row.get_unwrap(2),
                    added_at: row.get_unwrap(3),
                    last_modified: row.get_unwrap(4),
                    description: row.get_unwrap(5),
                    tags: self.get_bookmark_tags(&id),
                };
                Ok(item)
            })
            .unwrap()
            .map(|x| x.unwrap())
            .collect();

        Ok(matches)
    }

    pub fn get_bookmarks(
        &self,
        tags: &[String],
        domains: &[String],
    ) -> Result<Vec<Bookmark>, SimpleError> {
        let mut select_statement: String = "SELECT * FROM `Bookmark` WHERE ".into();

        if !tags.is_empty() {
            select_statement += &format!(
                "id in (SELECT DISTINCT bookmark_id FROM `Tag` WHERE tag in ({}))",
                tags.iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if !tags.is_empty() && !domains.is_empty() {
            select_statement += " AND ";
        }
        if !domains.is_empty() {
            let domains = domains
                .iter()
                .map(|x| format!("link LIKE '%{}%'", x))
                .collect::<Vec<String>>();
            select_statement += &format!("({})", domains.join(" OR "));
        }

        let mut select_statement = self.conn.prepare(&select_statement).unwrap();

        let matches: Vec<Bookmark> = select_statement
            .query_map([], |row| {
                let id: String = row.get_unwrap(0);
                let item = Bookmark {
                    id: row.get_unwrap(0),
                    name: row.get_unwrap(1),
                    link: row.get_unwrap(2),
                    added_at: row.get_unwrap(3),
                    last_modified: row.get_unwrap(4),
                    description: row.get_unwrap(5),
                    tags: self.get_bookmark_tags(&id),
                };
                Ok(item)
            })
            .unwrap()
            .map(|x| x.unwrap())
            .collect();

        Ok(matches)
    }

    pub fn update_bookmark_name(
        &self,
        bookmark: &Bookmark,
        new_name: &str,
    ) -> Result<(), SimpleError> {
        let query = format!(
            "UPDATE `Bookmark` SET name = \"{}\", last_modified = {} WHERE id LIKE {};",
            Regex::new("[\"']").unwrap().replace_all(new_name, "''"),
            Utc::now().timestamp(),
            bookmark.id
        );

        match self.conn.execute(&query, []) {
            Ok(_) => info!("Renamed '{}' to '{}'", bookmark.name, new_name),
            Err(e) => bail!("ERROR: Failed to update databse: {}", e),
        }

        Ok(())
    }

    pub fn update_bookmark_link(
        &self,
        bookmark: &Bookmark,
        new_link: &str,
    ) -> Result<(), SimpleError> {
        let query = format!(
            "UPDATE `Bookmark` SET link = \"{}\", last_modified = {} WHERE id LIKE {};",
            Regex::new("[\"']").unwrap().replace_all(new_link, "''"),
            Utc::now().timestamp(),
            bookmark.id
        );

        match self.conn.execute(&query, []) {
            Ok(_) => info!("Changed link from '{}' to '{}'", bookmark.link, new_link),
            Err(e) => bail!("ERROR: Failed to update databse: {}", e),
        }

        Ok(())
    }

    pub fn update_bookmark_description(
        &self,
        bookmark: &Bookmark,
        new_description: &str,
    ) -> Result<(), SimpleError> {
        let query =
            format!(
            "UPDATE `Bookmark` SET description = \"{}\", last_modified = '{}' WHERE id LIKE {};",
            Regex::new("[\"']").unwrap().replace_all(new_description, "''"),
            Utc::now().timestamp(),
            bookmark.id
        );

        match self.conn.execute(&query, []) {
            Ok(_) => info!(
                "Changed description from '{}' to '{}'",
                bookmark.description, new_description
            ),
            Err(e) => bail!("ERROR: Failed to update databse: {}", e),
        }

        Ok(())
    }

    pub fn update_bookmark_tags(
        &self,
        bookmark: &Bookmark,
        new_tags: &[String],
    ) -> Result<(), SimpleError> {
        let old_tags: HashSet<_> = bookmark.tags.0.iter().collect();
        let new_tags: HashSet<_> = new_tags.iter().collect();

        let added_tags: Vec<_> = new_tags.difference(&old_tags).collect();
        let deleted_tags = old_tags
            .difference(&new_tags)
            .map(|x| format!("'{}'", x))
            .join(", ");

        let delete_query = format!(
            "DELETE FROM `Tag` WHERE bookmark_id LIKE {} AND tag IN ({})",
            bookmark.id, deleted_tags
        );
        match self.conn.execute(&delete_query, []) {
            Ok(_) => info!("Deleted tags {}", deleted_tags),
            Err(e) => bail!("ERROR: Failed to update databse: {}", e),
        }

        for tag in added_tags {
            match self.conn.execute(
                "INSERT INTO `Tag` (bookmark_id, tag) VALUES (?1, ?2)",
                params![
                    bookmark.id,
                    Regex::new("[\"']").unwrap().replace_all(tag, "''"),
                ],
            ) {
                Ok(_) => info!("Added tag '{}' to '{}'", tag, bookmark.name),
                Err(_) => error!("Failed to add tag '{}' to '{}'", tag, bookmark.name),
            };
        }

        Ok(())
    }

    pub fn get_bookmark_tags(&self, bookmark_id: &str) -> TagList {
        let mut tag_query = self
            .conn
            .prepare("SELECT tag FROM `Tag` WHERE bookmark_id LIKE ?1")
            .unwrap();
        tag_query
            .query_map([bookmark_id], |x| x.get(0))
            .unwrap()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>()
            .into()
    }

    pub fn delete_tag(&self, tag: &str) -> Result<usize, SimpleError> {
        let query = "DELETE FROM `Tag` WHERE tag LIKE ?1";
        match self.conn.execute(query, [tag]) {
            Ok(c) => {
                info!("Removed {} items with tag {}.", c, tag);
                Ok(c)
            }
            Err(e) => bail!("ERROR: Failed to remove tag: {}", e),
        }
    }

    pub fn rename_tag(&self, old: &str, new: &str) -> Result<usize, SimpleError> {
        let query = format!(
            "UPDATE `Tag` SET tag = \"{}\" WHERE tag LIKE \"{}\"",
            new, old
        );
        match self.conn.execute(&query, []) {
            Ok(c) => {
                info!("Renamed {} items to {}.", c, new);
                Ok(c)
            }
            Err(e) => bail!("ERROR: Failed to rename tag: {}", e),
        }
    }
}
