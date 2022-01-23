use bkmrk_lib::BkmrkMan;
use clap::ArgMatches;
use color_eyre::Result;

pub fn exec_tag(args: &ArgMatches) -> Result<()> {
    let tag_name = args.value_of("tag-name").unwrap(); // Always has a value, thanks to clap

    let man = BkmrkMan::new();

    if args.is_present("delete") {
        let count = man.tag_delete(tag_name)?;
        println!("Deleted {tag_name} from {count} items.");
    }

    if args.is_present("rename") {
        let new_tag_name = args.value_of("rename").unwrap(); // Always has a value, thanks to clap
        let count = man.tag_rename(tag_name, new_tag_name)?;
        println!("Renamed {tag_name} to {new_tag_name} in {count} items.");
    }

    Ok(())
}
