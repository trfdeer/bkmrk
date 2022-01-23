use bkmrk_lib::BkmrkMan;
use clap::ArgMatches;
use simple_error::{bail, SimpleError};

pub fn exec_tag(args: &ArgMatches) -> Result<(), SimpleError> {
    let tag_name = args.value_of("tag-name").unwrap();

    let man = BkmrkMan::new();

    if args.is_present("delete") {
        match man.tag_delete(tag_name) {
            Ok(c) => {
                println!("Deleted tag '{}' from {} bookmarks", tag_name, c);
                return Ok(());
            }
            Err(e) => bail!("ERROR: Failed to delete tag '{}'\n{}", tag_name, e),
        }
    }

    if args.is_present("rename") {
        let new_tag_name = args.value_of("rename").unwrap();
        match man.tag_rename(tag_name, new_tag_name) {
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
