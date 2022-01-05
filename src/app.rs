use clap::clap_app;
use simple_error::{bail, SimpleError};

#[macro_export]
macro_rules! run_subcommand {
    ($cmd:ident, $arg:expr,  $err_msg:literal) => {
        match $cmd($arg) {
            Ok(_) => (),
            Err(e) => bail!("ERROR: {}\n{}", $err_msg, e),
        }
    };
}

fn setup_logger(verbosity: u64) -> Result<(), fern::InitError> {
    let verbosity = match verbosity {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        _ => log::LevelFilter::Info,
    };
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(verbosity)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

pub fn run_app() -> Result<(), SimpleError> {
    use super::subcommands::{
        add::exec_add, delete::exec_delete, edit::exec_edit, import::exec_import, ls::exec_ls,
        tag::exec_tag,
    };

    let matches = clap_app!(bkmrk =>
        (version: "0.1")
        (about: "Bookmark Manager")
        (@arg verbose: -v --verbose ... "Enable verbose output")
        (@subcommand add =>
            (about: "Add a new bookmark")
            (@arg name: -n --name +takes_value +required "Bookmark name")
            (@arg link: -l --link +takes_value +required "Bookmark link")
            (@arg tags: -t --tags +takes_value ... "Bookmark tags")
            (@arg description: -d --description +takes_value "Bookmark description")
        )
        (@subcommand ls =>
            (about: "List available bookmarks")
            (@arg output_type: -o long("output-type") +takes_value "Specify output type. Values: [table(default), format-string].")
            (@arg format_string: -f long("format-string") +takes_value ... "Specify output format string. Ignored if output-type is not set to format-string.\nAvailable Options:\n    %n for name\n    %l for link\n    %a for add date\n    %m for modified date\n    %t for tags\n    %d for description\nDefault: %n - %l [%t]")
            (@arg tags: -t --tags +takes_value ... "Show bookmarks from these tags only")
            (@arg domains: -d --domains +takes_value ... "Show bookmarks from these sites only")
        )
        (@subcommand edit =>
            (about: "Edit a bookmark")
            (@arg tags: -t --tags +takes_value ... "Show bookmarks from these tags only")
            (@arg domains: -d --domains +takes_value ... "Show bookmarks from these sites only")
        )
        (@subcommand import =>
            (about: "Import bookmarks from a file")
            (@arg file_format: -f long("file-format") +takes_value "Input file format")
            (@arg dry_run: -d --dry "Do not import. Only show bookmarks to be imported")
            (@arg append_folder_tags: -l long("append-folder-tags") "Append bookmark folders as tags")
            (@arg input_file: +required "The file to import")
        )
         (@subcommand delete =>
            (about: "Delete Bookmarks")
            (@arg tags: -t --tags +takes_value ... "Show bookmarks from these tags only")
            (@arg domains: -d --domains +takes_value ... "Show bookmarks from these sites only")
        )
        (@subcommand tag =>
            (about: "Mange tags")
            (@arg tag_name: -t long("tag-name") +takes_value +required "Tag name")
            (@arg rename: -r --rename +takes_value "New tag name")
            (@arg delete: -d --delete "Delete tag")
        )
    )
    .get_matches();

    let verbosity = matches.occurrences_of("verbose");
    setup_logger(verbosity).unwrap();

    let (command, args) = matches.subcommand();
    let args = args.unwrap();

    match command {
        "add" => run_subcommand!(exec_add, args, "Failed running add command"),
        "ls" => run_subcommand!(exec_ls, args, "Failed running ls command"),
        "edit" => run_subcommand!(exec_edit, args, "Failed running edit command"),
        "import" => run_subcommand!(exec_import, args, "Failed running import command"),
        "delete" => run_subcommand!(exec_delete, args, "Failed running delete command"),
        "tag" => run_subcommand!(exec_tag, args, "Failed running tag command"),
        _ => {}
    }

    Ok(())
}
