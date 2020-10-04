use processor::pypi;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs;

use error::AppError;

mod error;

mod processor;
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

impl Settings {
    fn from_file(path: &str) -> Result<Settings, std::io::Error> {
        let data = fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&data)?;
        Ok(settings)
    }
}

pub fn run() -> Result<(), AppError> {
    let settings = Settings::from_file("data/settings.toml")?;

    for group_name in settings.groups {
        let title = settings.titles.get(&group_name).unwrap();
        dbg!(title);
        let items = match group_name.as_str() {
            "pypi" => pypi::process(settings.links.get("pypi").unwrap()),
            _ => processor::process_simple_group(&group_name)
        }?;
        dbg!(items);
        println!("Zzz")
    }

    // let pypi_items = pypi::process(settings.links.get("pypi").unwrap());

    // let reg = Handlebars::new();
    // let template = fs::read_to_string("readme.hbs").unwrap();
    // // let data = json!({ "data": data });
    // // let result = reg.render_template(template.as_str(), &data).unwrap();
    // // println!("{}", result);

    Ok(())
}
