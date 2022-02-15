use bkmrk_lib::{site_metadata::SiteMetadata, BkmrkMan};
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use owo_colors::OwoColorize;

pub struct UpdateArgs {
    tags: Vec<String>,
    domains: Vec<String>,
    yes: bool,
}

impl UpdateArgs {
    pub fn new(tags: Vec<String>, domains: Vec<String>, yes: bool) -> Self {
        Self { tags, domains, yes }
    }
}

pub fn run(args: UpdateArgs) -> Result<()> {
    let tags: Vec<String> = args.tags;
    let domains: Vec<String> = args.domains;

    let man = BkmrkMan::new();

    let items = man.get_bookmarks(&tags, &domains)?;
    let options: Vec<_> = items
        .iter()
        .map(|it| format!("● {} - ({})", it.metadata.title, it.link))
        .collect();

    println!("Select bookmark(s) to update (q to cancel):");
    let indices = MultiSelect::new().items(&options).interact()?;

    for idx in indices {
        let bm = &items[idx];
        let data = SiteMetadata::get_metadata(&bm.link)?;
        println!("Editing {}", bm.link.green());

        if args.yes
            || prompt(&format!(
                "Update title from \"{}\" to \"{}\"?",
                bm.metadata.title, data.title
            ))
        {
            man.update_bookmark_name(bm, &data.title)?
        }

        if args.yes
            || prompt(&format!(
                "Update description from \"{}\" to \"{}\"?",
                bm.metadata
                    .description
                    .as_ref()
                    .unwrap_or(&String::default()),
                data.description.as_ref().unwrap_or(&String::default())
            ))
        {
            man.update_bookmark_descr(bm, &data.description.to_owned().unwrap_or_default())?
        }

        if args.yes
            || prompt(&format!(
                "Update image url from \"{}\" to \"{}\"?",
                bm.metadata.image_url.as_ref().unwrap_or(&String::default()),
                data.image_url.as_ref().unwrap_or(&String::default())
            ))
        {
            man.update_bookmark_image_url(
                bm,
                data.image_url.as_ref().unwrap_or(&String::default()),
            )?
        }

        if args.yes
            || prompt(&format!(
                "Update site type from \"{}\" to \"{}\"?",
                bm.metadata.site_type, data.site_type
            ))
        {
            man.update_bookmark_site_type(bm, &data.site_type.to_string())?
        }
    }

    Ok(())
}

fn prompt(msg: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact()
        .unwrap_or(false)
}
