use serde_json::Value;
use std::{collections::HashMap, fs, sync::mpsc::channel, thread};

use crate::{error::AppError, Item};

const PYPI_LIST_PATH: &str = "data/pypi_list.txt";
const PYPI_DESCRIPTION_PATH: &str = "data/pypi_description.toml";
const DESCRIPTION_WORKERS: usize = 5; // how match requests for description at once
const PYPI_API: &str = "https://pypi.org/pypi/${package}/json";

pub fn process() -> Result<Vec<Item>, AppError> {
    let packages = process_pypi_list()?;
    process_pypi_description(packages)?;
    todo!()
}

fn process_pypi_description(packages: Vec<String>) -> Result<(), AppError> {
    let descriptions_str = fs::read_to_string(PYPI_DESCRIPTION_PATH)?;
    let exists_desc: HashMap<String, String> = toml::from_str(descriptions_str.as_str()).unwrap();

    let packages_without_desription: Vec<String> =
        packages.into_iter().filter(|x| !exists_desc.contains_key(x)).collect();

    let chunks: Vec<Vec<String>> = packages_without_desription
        .chunks(DESCRIPTION_WORKERS)
        .map(|x| x.to_vec())
        .collect();

    for chunk in chunks {
        let (tx, rx) = channel();

        for package in chunk {
            let tx = tx.clone();
            thread::spawn(move || tx.send(get_package_description(package.to_string())).unwrap());
        }
        drop(tx); // It's important!

        let res: Vec<_> = rx.iter().collect();
        println!("{:?}", res);
    }

    Ok(())
}

fn process_pypi_list() -> Result<Vec<String>, AppError> {
    let data = fs::read_to_string(PYPI_LIST_PATH)?;

    // Trim, remove dublicates and sort
    let mut arr: Vec<_> = data
        .split("\n")
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();
    arr.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    arr.dedup();
    println!("{}", arr.join("\n"));

    fs::write(PYPI_LIST_PATH, arr.join("\n"))?;

    Ok(arr)
}

pub fn get_package_description(package: String) -> Result<String, AppError> {
    let url = PYPI_API.replace("${package}", &package);
    let res = reqwest::blocking::get(&url)?.json::<Value>()?;
    let description = res
        .pointer("/info/summary")
        .ok_or(AppError::PyPISummaryError)?
        .as_str()
        .ok_or(AppError::PyPISummaryError)?;
    Ok(description.to_string())
}
