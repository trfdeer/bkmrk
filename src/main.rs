use color_eyre::Result;

mod app;
mod subcommands;
mod utils;

fn main() -> Result<()> {
    color_eyre::install()?;
    app::run_app()?;
    Ok(())
}
