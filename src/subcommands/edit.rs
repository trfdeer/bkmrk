use clap::ArgMatches;
use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Input, Select};
use simple_error::{bail, SimpleError};

use crate::{bookmark::Bookmark, db::Database, utils};

pub fn exec_edit(args: &ArgMatches) -> Result<(), SimpleError> {
    let tags: Vec<String> = args
        .values_of("tags")
        .unwrap_or_default()
        .map(String::from)
        .collect();
    let domains: Vec<String> = args
        .values_of("domains")
        .unwrap_or_default()
        .map(String::from)
        .collect();

    let items = match Bookmark::get_matching_bookmarks(&tags, &domains) {
        Ok(r) => r,
        Err(e) => bail!("ERROR: Failed running edit command.\n{}", e),
    };

    let options: Vec<_> = items
        .iter()
        .map(|it| format!("● {} - ({})", it.name, it.link))
        .collect();

    println!("Select a bookmark to edit (q to cancel):");
    if let Some(index) = Select::with_theme(&ColorfulTheme::default())
        .items(&options)
        .interact_on_opt(&Term::stderr())
        .unwrap()
    {
        let db_path = utils::files::get_db_path().unwrap();
        let db = Database::connect(&db_path).unwrap();

        let editing = &items[index];

        if prompt("Update name?") {
            let new_name = get_input("Enter new name", &editing.name);
            match db.update_bookmark_name(editing, &new_name) {
                Ok(_) => println!("Name updated to '{}'!", new_name),
                Err(e) => bail!("ERROR: Failed to update name.\n{}", e),
            }
        }
        if prompt("Update link?") {
            let new_link = get_input("Enter new link", &editing.link);
            match db.update_bookmark_link(editing, &new_link) {
                Ok(_) => println!("Link updated to '{}'!", new_link),
                Err(e) => bail!("ERROR: Failed to update link.\n{}", e),
            }
        }
        if prompt("Update description?") {
            let new_description = get_input("Enter new description", &editing.description);
            match db.update_bookmark_description(editing, &new_description) {
                Ok(_) => println!("Description updated to '{}'!", new_description),
                Err(e) => bail!("ERROR: Failed to update description.\n{}", e),
            }
        }
        if prompt("Update tags?") {
            let new_tags = get_input(
                "Enter new tags (separated by commas)",
                &editing.tags.0.join(", "),
            );
            let new_tags = new_tags
                .split(',')
                .map(|x| x.trim().to_owned())
                .collect::<Vec<_>>();
            match db.update_bookmark_tags(editing, &new_tags) {
                Ok(_) => println!("Tags updated to '{}'!", new_tags.join(", ")),
                Err(e) => bail!("ERROR: Failed to update tags.\n{}", e),
            }
        }
    }

    Ok(())
}

fn get_input(prompt: &str, default_val: &str) -> String {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .with_initial_text(default_val)
        .interact_text()
        .unwrap_or_else(|_| default_val.into())
}

fn prompt(msg: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact()
        .unwrap_or(false)
}
