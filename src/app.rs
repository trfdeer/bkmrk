use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "bkmrk")]
#[clap(about = "A bookmark manager")]
pub struct App {
    #[clap(short, long)]
    /// Enable verbose output
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new bookmark
    Add {
        #[clap(short, long)]
        /// Bookmark Name
        name: String,

        #[clap(short, long)]
        /// Bookmark Link
        link: String,

        #[clap(short, long)]
        /// Bookmark Tags
        tags: Vec<String>,

        #[clap(short, long)]
        /// Bookmark Description
        description: Option<String>,
    },
    /// List available bookmarks
    List {
        #[clap(short, long, default_value_t = String::from("table"))]
        /// Specify output type. Values: [table(default), format-string]
        output_type: String,

        #[clap(short, long, default_value_t = String::from("%n - %l [%t]"))]
        /// Specify output format string. Ignored if output-type is not set to format-string.
        /// Available Options:
        ///     %n for name,
        ///     %l for link,
        ///     %a for add date,
        ///     %m for modified date,
        ///     %t for tags,
        ///     %d for description.
        /// Default: %n - %l [%t]
        format_string: String,

        #[clap(short, long)]
        /// Show bookmarks from these tags only
        tags: Vec<String>,

        #[clap(short, long)]
        /// Show bookmarks from these sites only
        domains: Vec<String>,
    },
    /// Edit a bookmark
    Edit {
        #[clap(short, long)]
        /// Show bookmarks from these tags only
        tags: Vec<String>,

        #[clap(short, long)]
        /// Show bookmarks from these sites only
        domains: Vec<String>,
    },
    /// Import bookmarks from a file
    Import {
        input_file: String,
        #[clap(short, long, default_value_t = String::from("netscape"))]
        /// Input file format
        file_format: String,

        #[clap(short, long)]
        /// Do not import. Only show bookmarks to be imported
        dry_run: bool,

        #[clap(short = 'l')]
        /// Append bookmark folders as tags
        append_folder_tags: bool,
    },
    /// Delete Bookmarks
    Delete {
        #[clap(short, long)]
        /// Show bookmarks from these tags only
        tags: Vec<String>,

        #[clap(short, long)]
        /// Show bookmarks from these sites only
        domains: Vec<String>,
    },
    /// Manage tags
    Tag {
        /// Name of tag to manage
        name: String,

        #[clap(short, long)]
        /// New tag name to rename to
        rename: Option<String>,

        #[clap(short, long)]
        /// Delete tag
        delete: bool,
    },
}

pub fn setup_logger(verbose: bool) -> Result<(), fern::InitError> {
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
        .level(if verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Error
        })
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
