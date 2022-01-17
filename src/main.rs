use std::process::exit;

mod app;
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
