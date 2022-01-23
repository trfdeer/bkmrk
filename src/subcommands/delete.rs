use bkmrk_lib::BkmrkMan;

use color_eyre::Result;
use dialoguer::MultiSelect;

pub struct DeleteArgs {
    tags: Vec<String>,
    domains: Vec<String>,
}

impl DeleteArgs {
    pub fn new(tags: Vec<String>, domains: Vec<String>) -> Self {
        Self { tags, domains }
    }
}

pub fn run(args: DeleteArgs) -> Result<()> {
    let tags: Vec<String> = args.tags;
    let domains: Vec<String> = args.domains;

    let man = BkmrkMan::new();

    let items = man.get_bookmarks(&tags, &domains)?;

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
