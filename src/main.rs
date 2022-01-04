use std::process::exit;

mod app;
mod bookmark;
mod db;
mod netscape_bookmark_parser;
mod subcommands;
mod utils;

fn main() {
    match app::run_app() {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
