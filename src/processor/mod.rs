mod npm;
pub mod pypi;
mod rcrate;
mod simple;

use std::fs;

use serde_json::json;

use crate::{error::AppError, Item, Items};
use std::collections::HashMap;

fn process_list_file(path: &str) -> Result<Vec<String>, AppError> {
    let data = fs::read_to_string(path)?;

    // Trim, remove dublicates and sort
    let mut arr: Vec<_> = data
        .split("\n")
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();
    arr.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    arr.dedup();

    fs::write(path, arr.join("\n"))?;

    Ok(arr)
}

pub fn process_simple_group(group: &str) -> Result<Vec<Item>, AppError> {
    let file = format!("data/{}.toml", group);
    dbg!(&file);
    let items: Items = toml::from_str(fs::read_to_string(&file)?.as_str())?;
    dbg!(&items);

    let mut items: Vec<Item> = items.items.into_iter().filter(|x| !x.name.is_empty()).collect();
    items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    fs::write(&file, toml::to_string(&json!({ "items": &items }))?)?;

    Ok(items)
}
