use std::path::PathBuf;

use bkmrk_lib::BkmrkMan;
use color_eyre::{eyre::eyre, Result};

use crate::utils;

pub struct ImportArgs {
    input_file: String,
    file_format: String,
    dry_run: bool,
    append_folder_tags: bool,
}

impl ImportArgs {
    pub fn new(
        input_file: String,
        file_format: String,
        dry_run: bool,
        append_folder_tags: bool,
    ) -> Self {
        Self {
            append_folder_tags,
            dry_run,
            file_format,
            input_file,
        }
    }
}

pub fn run(args: ImportArgs) -> Result<()> {
    let file_path = PathBuf::from(args.input_file);

    let man = BkmrkMan::new();

    if args.dry_run {
        let bookmarks = man.read_bookmark_file(&file_path, args.append_folder_tags)?;
        let terminal_dims =
            terminal_size::terminal_size().ok_or(eyre!("Couldn't get terminal size"))?;
        let terminal_dims = (terminal_dims.0 .0 as usize, terminal_dims.1 .0 as usize);
        println!("{}", utils::get_bookmark_table(&bookmarks, terminal_dims))
    } else {
        match args.file_format.as_str() {
            "netscape" => {
                let (succeeded, failed) =
                    man.import_bookmark_file(&file_path, args.append_folder_tags)?;
                println!("Bookmarks imported.");
                println!("{succeeded} Succeeded. {failed} Failed.")
            }
            _ => {
                return Err(eyre!(
                    "ERROR: File format {} invalid / unsupported.",
                    args.file_format
                ))
            }
        }
    }
    Ok(())
}
