#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate serde_json;

use std::fs;

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

#[derive(Deserialize, Debug)]
struct DataRoot {
    data: Vec<Data>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    name: String,
    tags: Vec<String>,
    link: String,
    description: String,
    created_at: Datetime,
}

pub fn read_data() -> Vec<Data> {
    let data = fs::read_to_string("data.toml").unwrap();
    let res: DataRoot = toml::from_str(data.as_str()).unwrap();

    // remove with empty name
    let mut data: Vec<_> = res.data.into_iter().filter(|d| !d.name.is_empty()).collect();
    // data.sort_by(|a, b| b.cmp_by_created_at(&a));
    data.sort_by(|a, b| b.created_at.to_string().cmp(&a.created_at.to_string()));
    return data;
}

pub fn generate_readme(data: Vec<Data>) {
    handlebars_helper!(tags: |v: array| {
    let d: Vec<String> = v.iter().map(|x|format!("#{}", x.as_str().unwrap())).collect();
    d.join(", ")
    });
    let mut reg = Handlebars::new();
    reg.register_helper("tags", Box::new(tags));
    let template = fs::read_to_string("readme.hbs").unwrap();
    let data = json!({ "data": data });
    let result = reg.render_template(template.as_str(), &data).unwrap();
    println!("{}", result);
}
