use std::path::PathBuf;

use bkmrk_lib::BkmrkMan;
use clap::ArgMatches;
use simple_error::{bail, SimpleError};

use crate::utils;

pub fn exec_import(args: &ArgMatches) -> Result<(), SimpleError> {
    let dry_run: bool = args.is_present("dry_run");
    let append_folder_tags = args.is_present("append_folder_tags");
    let file_path = PathBuf::from(args.value_of("input_file").unwrap());
    let file_format = args.value_of("file_format").unwrap_or("netscape");

    let man = BkmrkMan::new();

    if dry_run {
        let bookmarks = match man.read_bookmark_file(&file_path, append_folder_tags) {
            Ok(t) => t,
            Err(e) => {
                bail!(
                    "ERROR: Couldn't parse input file `{}`\n{}",
                    file_path.display(),
                    e
                )
            }
        };
        let terminal_dims = terminal_size::terminal_size().unwrap();
        let terminal_dims = (terminal_dims.0 .0 as usize, terminal_dims.1 .0 as usize);
        println!("{}", utils::get_bookmark_table(&bookmarks, terminal_dims))
    } else {
        match file_format {
            "netscape" => match man.import_bookmark_file(&file_path, append_folder_tags) {
                Ok((succeeded, failed)) => {
                    println!("Bookmarks imported.");
                    println!("{succeeded} Succeeded. {failed} Failed.")
                }
                Err(e) => bail!(
                    "ERROR: Failed to import input file {}\n{}",
                    file_path.display(),
                    e
                ),
            },
            _ => bail!("ERROR: File format {} invalid / unsupported.", file_format),
        }
    }
    Ok(())
}
