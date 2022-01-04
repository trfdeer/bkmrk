use std::path::{Path, PathBuf};

use clap::ArgMatches;
use simple_error::{bail, SimpleError};

use crate::{db::Database, netscape_bookmark_parser, utils};

pub fn exec_import(args: &ArgMatches) -> Result<(), SimpleError> {
    let dry_run: bool = args.is_present("dry_run");
    let append_folder_tags = args.is_present("append_folder_tags");
    let file_path: PathBuf = PathBuf::from(args.value_of("input_file").unwrap());
    let file_format = args.value_of("file_format").unwrap_or("netscape");

    match file_format {
        "netscape" => {
            match import_netscape_bookmark_file(&file_path, append_folder_tags, dry_run) {
                Ok(_) => println!("Bookmarks imported."),
                Err(e) => bail!(
                    "ERROR: Failed to import input file {}\n{}",
                    file_path.display(),
                    e
                ),
            }
        }
        _ => bail!("ERROR: File format {} invalid / unsupported.", file_format),
    }
    Ok(())
}

fn import_netscape_bookmark_file(
    file_path: &Path,
    append_folder_tags: bool,
    dry_run: bool,
) -> Result<(), SimpleError> {
    let db_path = utils::files::get_db_path().unwrap();
    let db = Database::connect(&db_path).unwrap();

    let bookmarks =
        match netscape_bookmark_parser::parse_netscape_bookmark_file(file_path, append_folder_tags)
        {
            Ok(t) => t,
            Err(e) => {
                bail!(
                    "ERROR: Couldn't parse input file `{}`\n{}",
                    file_path.display(),
                    e
                )
            }
        };

    if !dry_run {
        let (succeeded, failed) = db.add_bookmarks(&bookmarks).unwrap();
        println!("{} Added. {} Failed", succeeded, failed);
    } else {
        for bookmark in bookmarks {
            println!("{}", bookmark);
        }
    }

    Ok(())
}
