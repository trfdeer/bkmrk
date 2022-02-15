#![deny(missing_docs)]

use clap::{ArgEnum, Parser, Subcommand};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum OutputType {
    Table,
    FormatString,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(visible_alias = "a")]
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

    #[clap(visible_alias = "ls")]
    /// List available bookmarks
    List {
        #[clap(short, long, arg_enum, default_value_t = OutputType::Table)]
        /// Specify output type
        output_type: OutputType,

        #[clap(short, long, default_value_t = String::from("%n - %l [%t]"))]
        /// Specify output format string. Ignored if output-type is not set to format-string.
        /// Available Options:
        /// %n for name,
        /// %l for link,
        /// %a for add date,
        /// %m for modified date,
        /// %t for tags,
        /// %d for description.
        format_string: String,

        #[clap(short, long)]
        /// Show bookmarks from these tags only
        tags: Vec<String>,

        #[clap(short, long)]
        /// Show bookmarks from these sites only
        domains: Vec<String>,
    },

    #[clap(visible_alias = "e")]
    /// Edit a bookmark
    Edit {
        #[clap(short, long)]
        /// Show bookmarks from these tags only
        tags: Vec<String>,

        #[clap(short, long)]
        /// Show bookmarks from these sites only
        domains: Vec<String>,
    },

    #[clap(visible_alias = "u")]
    /// Update bookmark details
    Update {
        #[clap(short, long)]
        /// Show bookmarks from these tags only
        tags: Vec<String>,

        #[clap(short, long)]
        /// Show bookmarks from these sites only
        domains: Vec<String>,

        #[clap(short, long)]
        /// Confirm without prompts
        yes: bool,
    },

    #[clap(visible_alias = "i")]
    /// Import bookmarks from a file
    Import {
        input_file: String,
        #[clap(short, long, default_value_t = String::from("netscape"))]
        /// Input file format
        file_format: String,

        #[clap(short, long)]
        /// Do not import. Only show bookmarks to be imported
        dry_run: bool,

        #[clap(short = 'l', long)]
        /// Append bookmark folders as tags
        append_folder_tags: bool,
    },

    #[clap(visible_alias = "d")]
    /// Delete Bookmarks
    Delete {
        #[clap(short, long)]
        /// Show bookmarks from these tags only
        tags: Vec<String>,

        #[clap(short, long)]
        /// Show bookmarks from these sites only
        domains: Vec<String>,
    },

    #[clap(visible_alias = "t")]
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
