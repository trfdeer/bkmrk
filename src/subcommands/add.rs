use bkmrk_lib::{
    bookmark::{Bookmark, TagList},
    site_metadata::SiteMetadata,
    BkmrkMan,
};
use color_eyre::Result;

pub struct AddArgs {
    name: String,
    link: String,
    tags: Vec<String>,
    description: Option<String>,
}

impl AddArgs {
    pub fn new(name: String, link: String, tags: Vec<String>, description: Option<String>) -> Self {
        Self {
            name,
            link,
            tags,
            description,
        }
    }
}

pub fn run(args: AddArgs) -> Result<()> {
    let name: String = args.name;
    let link: String = args.link;
    let tags: TagList = args.tags.into();
    let description: String = args.description.unwrap_or_else(|| "".into());

    let new_bookmark = Bookmark {
        link,
        tags,
        metadata: SiteMetadata {
            title: name,
            description: Some(description),
            ..Default::default()
        },
        ..Default::default()
    };

    let man = BkmrkMan::new();
    man.add_bookmark(&new_bookmark)?;

    Ok(())
}
