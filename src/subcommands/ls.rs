use crate::{app::OutputType, utils};
use bkmrk_lib::BkmrkMan;
use color_eyre::{eyre::eyre, Result};

pub struct ListArgs {
    output_type: OutputType,
    format_string: String,
    tags: Vec<String>,
    domains: Vec<String>,
}

impl ListArgs {
    pub fn new(
        output_type: OutputType,
        format_string: String,
        tags: Vec<String>,
        domains: Vec<String>,
    ) -> Self {
        Self {
            output_type,
            format_string,
            tags,
            domains,
        }
    }
}

pub fn run(args: ListArgs) -> Result<()> {
    let tags: Vec<String> = args.tags;
    let domains: Vec<String> = args.domains;

    let man = BkmrkMan::new();
    let items = man.get_bookmarks(&tags, &domains)?;
    println!("Got {} items.", items.len());

    match args.output_type {
        OutputType::Table => {
            let terminal_dims =
                terminal_size::terminal_size().ok_or(eyre!("Couldn't get terminal size"))?;
            let terminal_dims = (terminal_dims.0 .0 as usize, terminal_dims.1 .0 as usize);
            println!("{}", utils::get_bookmark_table(&items, terminal_dims))
        }
        OutputType::FormatString => {
            for it in items {
                println!("{}", it.format(&args.format_string));
            }
        }
    }

    Ok(())
}
