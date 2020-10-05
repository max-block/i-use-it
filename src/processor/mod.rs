use std::{collections::BTreeMap, collections::HashMap, fs, sync::mpsc::channel};

use serde_json::json;
use threadpool::ThreadPool;

use crate::{error::AppError, Item, Items};

pub mod crates;
pub mod npm;
pub mod pypi;

const DESCRIPTION_WORKERS: usize = 5;
type PackageDescFn = fn(String) -> Result<(String, String), AppError>;

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

fn process_description_file(
    packages: Vec<String>,
    description_file: &str,
    package_description_fn: PackageDescFn,
) -> Result<(), AppError> {
    let mut descriptions: BTreeMap<String, String> = toml::from_str(fs::read_to_string(description_file)?.as_str())?;

    let packages_without_description: Vec<String> =
        packages.into_iter().filter(|x| !descriptions.contains_key(x)).collect();

    let pool = ThreadPool::new(DESCRIPTION_WORKERS);
    let (tx, rx) = channel();
    for package in packages_without_description {
        let tx = tx.clone();
        pool.execute(move || {
            // tx.send(get_package_description(package.to_string())).unwrap();
            tx.send(package_description_fn(package.to_string())).unwrap();
        });
    }
    drop(tx);
    let res: Vec<_> = rx.iter().collect();
    for r in res {
        if let Ok(val) = r {
            descriptions.insert(val.0, val.1);
        }
    }

    fs::write(description_file, toml::to_string(&descriptions)?)?;
    Ok(())
}

fn process_description_group(link: &str, description_file: &str, group_file: &str) -> Result<Vec<Item>, AppError> {
    let descriptions: BTreeMap<String, String> = toml::from_str(fs::read_to_string(description_file)?.as_str())?;
    let items: Vec<Item> = descriptions
        .into_iter()
        .map(|(k, v)| Item {
            link: link.replace("${package}", &k),
            name: k,
            description: v,
        })
        .collect();
    let mut data: HashMap<&str, &Vec<Item>> = HashMap::new();
    data.insert("items", &items);
    fs::write(group_file, toml::to_string(&data)?)?;
    Ok(items)
}

pub fn process_simple_group(group: &str) -> Result<Vec<Item>, AppError> {
    let file = format!("data/{}.toml", group);
    let items: Items = toml::from_str(fs::read_to_string(&file)?.as_str())?;

    let mut items: Vec<Item> = items.items.into_iter().filter(|x| !x.name.is_empty()).collect();
    items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    fs::write(&file, toml::to_string(&json!({ "items": &items }))?)?;

    Ok(items)
}
