use bkmrk_lib::BkmrkMan;
use color_eyre::Result;
use owo_colors::OwoColorize;

pub struct TagArgs {
    name: Option<String>,
    list: bool,
    rename: Option<String>,
    delete: bool,
}

impl TagArgs {
    pub fn new(name: Option<String>, list: bool, rename: Option<String>, delete: bool) -> Self {
        Self {
            name,
            list,
            rename,
            delete,
        }
    }
}

pub fn run(args: TagArgs) -> Result<()> {
    let man = BkmrkMan::new();

    if args.list {
        let tag_counts = man.tag_counts()?;
        for (idx, tag) in tag_counts.into_iter().enumerate() {
            println!(
                "{:02}. {} {}",
                idx + 1,
                tag.0.green(),
                format!("x{}", tag.1).yellow()
            );
        }
    }

    if args.delete {
        let tag_name = args.name.as_ref().unwrap();
        let count = man.tag_delete(tag_name)?;
        println!("Deleted {tag_name} from {count} items.");
    }

    if let Some(new_tag_name) = args.rename {
        let tag_name = args.name.as_ref().unwrap();
        let count = man.tag_rename(tag_name, &new_tag_name)?;
        println!("Renamed {tag_name} to {new_tag_name} in {count} items.");
    }

    Ok(())
}
