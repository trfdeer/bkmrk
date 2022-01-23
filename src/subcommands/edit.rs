use bkmrk_lib::BkmrkMan;
use clap::ArgMatches;
use color_eyre::Result;
use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Input, Select};

pub fn exec_edit(args: &ArgMatches) -> Result<()> {
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

    let man = BkmrkMan::new();

    let items = man.get_bookmarks(&tags, &domains)?;

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
        let editing = &items[index];

        if prompt("Update name?") {
            let new_name = get_input("Enter new name", &editing.name);
            man.update_bookmark_name(editing, &new_name)?
        }
        if prompt("Update link?") {
            let new_link = get_input("Enter new link", &editing.link);
            man.update_bookmark_link(editing, &new_link)?;
        }
        if prompt("Update description?") {
            let new_description = get_input("Enter new description", &editing.description);
            man.update_bookmark_descr(editing, &new_description)?;
        }
        if prompt("Update tags?") {
            let new_tags = get_input(
                "Enter new tags (separated by commas)",
                &editing.tags.0.join(", "),
            )
            .split(',')
            .map(|x| x.trim().to_owned())
            .collect::<Vec<_>>();
            man.update_bookmark_tags(editing, &new_tags)?;
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
