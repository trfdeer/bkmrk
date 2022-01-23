use crate::utils;
use bkmrk_lib::BkmrkMan;
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};

pub fn exec_ls(args: &ArgMatches) -> Result<()> {
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

    let man = BkmrkMan::new();

    let items = man.get_bookmarks(&tags, &domains)?;

    println!("Got {} items.", items.len());

    let output_type = args.value_of("output_type").unwrap_or("table");
    match output_type {
        "table" => {
            let terminal_dims =
                terminal_size::terminal_size().ok_or(eyre!("Couldn't get terminal size"))?;
            let terminal_dims = (terminal_dims.0 .0 as usize, terminal_dims.1 .0 as usize);
            println!("{}", utils::get_bookmark_table(&items, terminal_dims))
        }
        "format-string" => {
            let format_string = args.value_of("format_string").unwrap_or("%n - %l [%t]");
            for it in items {
                println!("{}", it.format(format_string));
            }
        }
        e => return Err(eyre!("Invalid output type: {}", e)),
    }

    Ok(())
}
