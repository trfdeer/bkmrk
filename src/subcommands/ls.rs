use crate::bookmark::Bookmark;
use clap::ArgMatches;
use simple_error::{bail, SimpleError};

pub fn exec_ls(args: &ArgMatches) -> Result<(), SimpleError> {
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

    println!("Got {} items.", items.len());

    let output_type = args.value_of("output_type").unwrap_or("table");
    match output_type {
        "table" => println!("{}", Bookmark::get_table(&items)),
        "format-string" => {
            let format_string = args.value_of("format_string").unwrap_or("%n - %l [%t]");

            for it in items {
                println!("{}", unescape::unescape(&it.format(format_string)).unwrap());
            }
        }
        e => bail!("ERROR: Invalid output type: {}", e),
    }

    Ok(())
}
