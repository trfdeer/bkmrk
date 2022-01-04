use crate::{bookmark::Bookmark, db::Database, utils};
use clap::ArgMatches;

use dialoguer::MultiSelect;
use simple_error::{bail, SimpleError};

pub fn exec_delete(args: &ArgMatches) -> Result<(), SimpleError> {
    let tags: Vec<String> = args
        .values_of("tags")
        .unwrap_or_default()
        .map(String::from)
        .collect();
    let domains: Vec<String> = args
        .values_of("domains")
        .unwrap_or_default()
        .map(String::from)
        .collect();

    let items = match Bookmark::get_matching_bookmarks(&tags, &domains) {
        Ok(r) => r,
        Err(e) => bail!("ERROR: Failed running ls command.\n{}", e),
    };

    let options: Vec<_> = items
        .iter()
        .map(|it| format!("{} - ({})", it.name, it.link))
        .collect();

    println!("Select bookmarks to edit:");
    let indices = MultiSelect::new().items(&options).interact().unwrap();
    println!("{:?}", indices);

    let items = indices
        .iter()
        .map(|&idx| items[idx].to_owned())
        .collect::<Vec<_>>();

    let db_path = utils::files::get_db_path().unwrap();
    let db = Database::connect(&db_path).unwrap();

    let (succeeded, failed) = db.delete_bookmarks(&items).unwrap();
    println!("{} Deleted. {} Failed", succeeded, failed);

    Ok(())
}
