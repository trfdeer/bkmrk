use bkmrk_lib::BkmrkMan;
use color_eyre::Result;

pub struct TagArgs {
    name: String,
    rename: Option<String>,
    delete: bool,
}

impl TagArgs {
    pub fn new(name: String, rename: Option<String>, delete: bool) -> Self {
        Self {
            name,
            rename,
            delete,
        }
    }
}

pub fn run(args: TagArgs) -> Result<()> {
    let tag_name = args.name;
    let man = BkmrkMan::new();

    if args.delete {
        let count = man.tag_delete(&tag_name)?;
        println!("Deleted {tag_name} from {count} items.");
    }

    if let Some(new_tag_name) = args.rename {
        let count = man.tag_rename(&tag_name, &new_tag_name)?;
        println!("Renamed {tag_name} to {new_tag_name} in {count} items.");
    }

    Ok(())
}
