#[macro_use]
extern crate handlebars;

use std::collections::HashMap;
use std::fs;

use handlebars::{handlebars_helper, Handlebars};
use serde::{Deserialize, Serialize};
use serde_json::json;

use error::AppError;
use processor::pypi;

mod error;

mod processor;

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub link: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    groups: Vec<String>,
    titles: HashMap<String, String>,
    links: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    title: String,
    items: Vec<Item>,
}

impl Settings {
    fn from_file(path: &str) -> Result<Settings, std::io::Error> {
        let data = fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&data)?;
        Ok(settings)
    }
}

fn process_groups(settings: &Settings) -> Result<Vec<Group>, AppError> {
    let mut groups: Vec<Group> = vec![];
    for group_name in settings.groups.iter() {
        let title = settings.titles.get(group_name).unwrap().to_owned();
        let items = match group_name.as_str() {
            "pypi" => pypi::process(settings.links.get("pypi").unwrap()),
            _ => processor::process_simple_group(&group_name),
        }?;
        groups.push(Group { title, items })
    }
    Ok(groups)
}

fn update_readme(groups: Vec<Group>) -> Result<(), AppError> {
    handlebars_helper!(anchor: |v: str| format!("#{}", v.to_lowercase().replace(" ", "-")));
    let mut reg = Handlebars::new();
    reg.register_helper("anchor", Box::new(anchor));
    let template = fs::read_to_string("readme.hbs").unwrap();

    let result = reg
        .render_template(template.as_str(), &json!({ "groups": groups }))
        .unwrap();

    println!("{}", result);
    fs::write("README.md", result)?;
    Ok(())
}

pub fn run() -> Result<(), AppError> {
    let settings = Settings::from_file("data/settings.toml")?;
    update_readme(process_groups(&settings)?)?;

    println!("\n\ndone!");
    Ok(())
}
