use serde_json::Value;
use std::{collections::BTreeMap, collections::HashMap, fs, sync::mpsc::channel};
use threadpool::ThreadPool;

use crate::{error::AppError, Item};

const PYPI_LIST_FILE: &str = "data/pypi_list.txt";
const PYPI_DESCRIPTION_FILE: &str = "data/pypi_description.toml";
const PYPI_TOML_FILE: &str = "data/pypi.toml";
const DESCRIPTION_WORKERS: usize = 5; // how match requests for description at once
const PYPI_API: &str = "https://pypi.org/pypi/${package}/json";

pub fn process(link: &str) -> Result<Vec<Item>, AppError> {
    let packages = super::process_list_file(PYPI_LIST_FILE)?;
    process_descriptions(packages)?;
    process_group(link)
}

fn process_group(link: &str) -> Result<Vec<Item>, AppError> {
    let descriptions: BTreeMap<String, String> = toml::from_str(fs::read_to_string(PYPI_DESCRIPTION_FILE)?.as_str())?;
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
    fs::write(PYPI_TOML_FILE, toml::to_string(&data)?)?;
    Ok(items)
}

fn process_descriptions(packages: Vec<String>) -> Result<(), AppError> {
    let mut descriptions: BTreeMap<String, String> =
        toml::from_str(fs::read_to_string(PYPI_DESCRIPTION_FILE)?.as_str())?;

    let packages_without_desription: Vec<String> =
        packages.into_iter().filter(|x| !descriptions.contains_key(x)).collect();

    let pool = ThreadPool::new(DESCRIPTION_WORKERS);
    let (tx, rx) = channel();
    for package in packages_without_desription {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(get_package_description(package.to_string())).unwrap();
        });
    }
    drop(tx);
    let res: Vec<_> = rx.iter().collect();
    for r in res {
        if let Ok(val) = r {
            descriptions.insert(val.0, val.1);
        }
    }

    fs::write(PYPI_DESCRIPTION_FILE, toml::to_string(&descriptions)?)?;
    Ok(())
}

pub fn get_package_description(package: String) -> Result<(String, String), AppError> {
    let url = PYPI_API.replace("${package}", &package);
    let res = reqwest::blocking::get(&url)?.json::<Value>()?;
    let description = res
        .pointer("/info/summary")
        .ok_or(AppError::PyPISummaryError)?
        .as_str()
        .ok_or(AppError::PyPISummaryError)?;
    Ok((package, description.to_string()))
}
