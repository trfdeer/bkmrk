use clap::Parser;
use color_eyre::Result;
mod app;
mod subcommands;
mod utils;

use app::{App, Commands};

use subcommands::*;
use subcommands::{
    add::AddArgs, delete::DeleteArgs, edit::EditArgs, import::ImportArgs, ls::ListArgs,
    tag::TagArgs, update::UpdateArgs,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    dotenv::dotenv().ok();

    let args = App::parse();

    if args.verbose {
        match std::env::var("RUST_LOG") {
            Ok(c) => println!("Using log config: {c}"),
            Err(_) => std::env::set_var("RUST_LOG", "bkmrk=debug,bkmrk_lib=debug"),
        }
    }

    pretty_env_logger::init_timed();
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
        Commands::Update { domains, tags, yes } => {
            update::run(UpdateArgs::new(tags, domains, yes))?
        }
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
            list,
            rename,
            delete,
        } => tag::run(TagArgs::new(name, list, rename, delete))?,
    }

    Ok(())
}
