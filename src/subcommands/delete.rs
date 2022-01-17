use bkmrk_lib::BkmrkMan;
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

    let man = BkmrkMan::new();

    let items = match man.get_bookmarks(&tags, &domains) {
        Ok(r) => r,
        Err(e) => bail!("ERROR: Failed running ls command.\n{}", e),
    };

    let options: Vec<_> = items
        .iter()
        .map(|it| format!("{} - ({})", it.name, it.link))
        .collect();

    println!("Select bookmarks to edit:");
    let indices = MultiSelect::new().items(&options).interact().unwrap();

    let items = indices
        .iter()
        .map(|&idx| items[idx].to_owned())
        .collect::<Vec<_>>();

    let (succeeded, failed) = man.delete_bookmarks(&items).unwrap();
    println!("{} Deleted. {} Failed", succeeded, failed);

    Ok(())
}
