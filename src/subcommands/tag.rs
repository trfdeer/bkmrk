use clap::ArgMatches;
use simple_error::{bail, SimpleError};

use crate::{db::Database, utils};

pub fn exec_tag(args: &ArgMatches) -> Result<(), SimpleError> {
    let tag_name = args.value_of("tag_name").unwrap();

    let db_path = utils::files::get_db_path().unwrap();
    let db = Database::connect(&db_path).unwrap();

    if args.is_present("delete") {
        match db.delete_tag(tag_name) {
            Ok(c) => {
                println!("Deleted tag '{}' from {} bookmarks", tag_name, c);
                return Ok(());
            }
            Err(e) => bail!("ERROR: Failed to delete tag '{}'\n{}", tag_name, e),
        }
    }

    if args.is_present("rename") {
        let new_tag_name = args.value_of("rename").unwrap();
        match db.rename_tag(tag_name, new_tag_name) {
            Ok(c) => {
                println!(
                    "Renamed '{}' to '{}' on {} bookmarks.",
                    tag_name, new_tag_name, c
                );
                return Ok(());
            }
            Err(e) => bail!("ERROR: Failed to rename tags\n{}", e),
        }
    }

    Ok(())
}
