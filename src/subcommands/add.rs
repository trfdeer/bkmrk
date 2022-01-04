use chrono::Utc;
use clap::ArgMatches;
use simple_error::{bail, SimpleError};

use crate::{
    bookmark::{Bookmark, TagList},
    db::Database,
    utils,
};

pub fn exec_add(args: &ArgMatches) -> Result<(), SimpleError> {
    let name: String = args.value_of("name").unwrap().into();
    let link: String = args.value_of("link").unwrap().into();
    let tags: TagList = args
        .values_of("tags")
        .unwrap_or_default()
        .map(String::from)
        .collect::<Vec<_>>()
        .into();
    let description: String = args.value_of("description").unwrap_or_default().into();
    let time_now = Utc::now().timestamp();

    let new_bookmark = Bookmark {
        name,
        link,
        added_at: time_now,
        last_modified: time_now,
        tags,
        description,
        ..Default::default()
    };

    let db_path = utils::files::get_db_path().unwrap();
    let db = Database::connect(&db_path).unwrap();
    match db.add_bookmark(&new_bookmark) {
        Ok(_) => println!("Added bookmark."),
        Err(e) => bail!("ERROR: Failed to add bookmark\n{}", e),
    };

    Ok(())
}
