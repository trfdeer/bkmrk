use std::{collections::HashSet, fs::File, path::Path, rc::Rc};

use chrono::Utc;
use eyre::{Result, WrapErr};
use log::{error, info};
use nanoid::nanoid;
use rusqlite::{params, types::Value, Connection};

use crate::bookmark::{Bookmark, TagList};

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

#[allow(dead_code)]
impl Database {
    pub fn create_tables(&self) -> Result<()> {
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
        self.conn.execute_batch(query)?;
        Ok(())
    }

    pub fn connect(path: &Path) -> Result<Self> {
        let mut new_db = false;
        if !path.exists() {
            new_db = true;
            File::create(path)
                .wrap_err_with(|| format!("Couldn't create database file at {}", path.display()))?;
        }
        let conn = Connection::open(path).wrap_err_with(|| format!("Couldn't connect to db"))?;
        let db = Self { conn };

        if new_db {
            db.create_tables()
                .wrap_err_with(|| format!("Couldn't create database tables"))?;
        }
        Ok(db)
    }

    pub fn add_one(&self, bookmark: &Bookmark) -> Result<()> {
        let id = nanoid!(
            6,
            &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f']
        );
        self.conn.execute(
            "INSERT INTO `Bookmark` (id, name, link, added_at, last_modified, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, bookmark.name, bookmark.link, bookmark.added_at, bookmark.last_modified, bookmark.description],
        )?;

        for tag in &bookmark.tags.0 {
            self.conn.execute(
                "INSERT INTO `Tag` (bookmark_id, tag) VALUES (?1, ?2)",
                params![id, tag],
            )?;
        }

        Ok(())
    }

    pub fn add_many(&self, bookmarks: &[Bookmark]) -> Result<(usize, usize)> {
        let (mut succeeded, mut failed) = (0, 0);
        for bookmark in bookmarks {
            match self.add_one(bookmark) {
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

    pub fn delete_one(&self, bookmark_id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM `Tag` WHERE bookmark_id LIKE ?1", [bookmark_id])
            .wrap_err("Couldn't bookmark")?;

        self.conn
            .execute("DELETE FROM `Bookmark` WHERE id LIKE ?1", [bookmark_id])
            .wrap_err("Couldn't delete related tags")?;

        Ok(())
    }

    pub fn delete_many(&self, bookmarks: &[Bookmark]) -> Result<(usize, usize)> {
        let (mut succeeded, mut failed) = (0, 0);
        for bookmark in bookmarks {
            match self.delete_one(&bookmark.id) {
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

    pub fn get_all(&self) -> Result<Vec<Bookmark>> {
        let mut select_statement = self.conn.prepare("SELECT * FROM `Bookmark`")?;
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
                    tags: self
                        .get_tags(&id)
                        .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?,
                };
                Ok(item)
            })?
            .collect::<Result<Vec<Bookmark>, _>>()?;

        Ok(matches)
    }

    pub fn get(&self, tags: &[String], domains: &[String]) -> Result<Vec<Bookmark>> {
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

        let mut select_statement = self.conn.prepare(&select_statement)?;

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
                    tags: self
                        .get_tags(&id)
                        .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?,
                };
                Ok(item)
            })
            .unwrap()
            .map(|x| x.unwrap())
            .collect();

        Ok(matches)
    }

    pub fn update_name(&self, bookmark: &Bookmark, new_name: &str) -> Result<()> {
        let query = "UPDATE `Bookmark` SET name = ?1, last_modified = ?2 WHERE id LIKE ?3;";
        self.conn.execute(
            query,
            [new_name, &Utc::now().timestamp().to_string(), &bookmark.id],
        )?;

        Ok(())
    }

    pub fn update_link(&self, bookmark: &Bookmark, new_link: &str) -> Result<()> {
        let query = "UPDATE `Bookmark` SET link = ?1, last_modified = ?2 WHERE id LIKE ?3;";

        self.conn.execute(
            query,
            [new_link, &Utc::now().timestamp().to_string(), &bookmark.id],
        )?;

        Ok(())
    }

    pub fn update_descr(&self, bookmark: &Bookmark, new_description: &str) -> Result<()> {
        let query = "UPDATE `Bookmark` SET description = ?1, last_modified = ?2 WHERE id LIKE ?3;";
        self.conn.execute(
            query,
            [
                new_description,
                &Utc::now().timestamp().to_string(),
                &bookmark.id,
            ],
        )?;

        Ok(())
    }

    pub fn update_tags(&self, bookmark: &Bookmark, new_tags: &[String]) -> Result<(usize, usize)> {
        rusqlite::vtab::array::load_module(&self.conn)?;

        let old_tags: HashSet<_> = bookmark.tags.0.iter().collect();
        let new_tags: HashSet<_> = new_tags.iter().collect();

        let added_tags: Vec<_> = new_tags.difference(&old_tags).collect();
        let deleted_tags = old_tags.difference(&new_tags);

        let values = Rc::new(
            deleted_tags
                .copied()
                .map(|s| Value::from(s.to_owned()))
                .collect::<Vec<Value>>(),
        );

        let mut delete_query = self
            .conn
            .prepare("DELETE FROM `Tag` WHERE bookmark_id LIKE ?1 AND tag IN rarray(?)")?;

        let delete_count = delete_query.execute(params![bookmark.id, values])?;

        let mut add_count = 0;
        for tag in added_tags {
            match self.conn.execute(
                "INSERT INTO `Tag` (bookmark_id, tag) VALUES (?1, ?2)",
                params![bookmark.id, tag],
            ) {
                Ok(_) => {
                    info!("Added tag '{}' to '{}'", tag, bookmark.name);
                    add_count += 1;
                }
                Err(_) => error!("Failed to add tag '{}' to '{}'", tag, bookmark.name),
            };
            add_count += 1;
        }

        Ok((add_count, delete_count))
    }

    fn get_tags(&self, bookmark_id: &str) -> Result<TagList> {
        let mut tag_query = self
            .conn
            .prepare("SELECT tag FROM `Tag` WHERE bookmark_id LIKE ?1")?;
        let tags = tag_query
            .query_map([bookmark_id], |x| x.get(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(TagList(tags))
    }

    pub fn tag_delete(&self, tag: &str) -> Result<usize> {
        let query = "DELETE FROM `Tag` WHERE tag LIKE ?1";
        let count = self.conn.execute(query, [tag])?;
        Ok(count)
    }

    pub fn tag_rename(&self, old: &str, new: &str) -> Result<usize> {
        let query = "UPDATE `Tag` SET tag = ?1 WHERE tag LIKE ?2";
        let count = self.conn.execute(query, [new, old])?;
        Ok(count)
    }
}
