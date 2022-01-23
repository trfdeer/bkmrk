use clap::Parser;
use color_eyre::Result;
mod app;
mod subcommands;
mod utils;

use app::{setup_logger, App, Commands};
use subcommands::*;

use crate::subcommands::{
    add::AddArgs, delete::DeleteArgs, edit::EditArgs, import::ImportArgs, ls::ListArgs,
    tag::TagArgs,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = App::parse();
    setup_logger(args.verbose)?;
    run_app(args.command)?;
    Ok(())
}

pub fn run_app(command: Commands) -> Result<()> {
    match command {
        Commands::Add {
            description,
            link,
            name,
            tags,
        } => add::run(AddArgs::new(name, link, tags, description))?,
        Commands::List {
            domains,
            format_string,
            output_type,
            tags,
        } => ls::run(ListArgs::new(output_type, format_string, tags, domains))?,
        Commands::Edit { domains, tags } => edit::run(EditArgs::new(tags, domains))?,
        Commands::Import {
            input_file,
            append_folder_tags,
            dry_run,
            file_format,
        } => import::run(ImportArgs::new(
            input_file,
            file_format,
            dry_run,
            append_folder_tags,
        ))?,
        Commands::Delete { tags, domains } => delete::run(DeleteArgs::new(tags, domains))?,
        Commands::Tag {
            name,
            rename,
            delete,
        } => tag::run(TagArgs::new(name, rename, delete))?,
    }

    Ok(())
}
