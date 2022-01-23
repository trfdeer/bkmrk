use bkmrk_lib::{
    bookmark::{Bookmark, TagList},
    BkmrkMan,
};
use clap::ArgMatches;
use color_eyre::Result;

pub fn exec_add(args: &ArgMatches) -> Result<()> {
    let name: String = args.value_of("name").unwrap().into();
    let link: String = args.value_of("link").unwrap().into();
    let tags: TagList = args
        .values_of("tags")
        .unwrap_or_default()
        .map(String::from)
        .collect::<Vec<_>>()
        .into();
    let description: String = args.value_of("description").unwrap_or_default().into();

    let new_bookmark = Bookmark {
        name,
        link,
        tags,
        description,
        ..Default::default()
    };

    let man = BkmrkMan::new();
    man.add_bookmark(&new_bookmark)?;

    Ok(())
}
