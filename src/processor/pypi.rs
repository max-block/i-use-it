use serde_json::Value;

use crate::{error::AppError, Item};

const LIST_FILE: &str = "data/pypi_list.txt";
const DESCRIPTION_FILE: &str = "data/pypi_description.toml";
const GROUP_FILE: &str = "data/pypi.toml";
const API_URL: &str = "https://pypi.org/pypi/${package}/json";

pub fn process(link: &str) -> Result<Vec<Item>, AppError> {
    let packages = super::process_list_file(LIST_FILE)?;
    super::process_description_file(packages, DESCRIPTION_FILE, get_package_description)?;
    super::process_description_group(link, DESCRIPTION_FILE, GROUP_FILE)
}

pub fn get_package_description(package: String) -> Result<(String, String), AppError> {
    let url = API_URL.replace("${package}", &package);
    let res = reqwest::blocking::get(&url)?.json::<Value>()?;
    let description = res
        .pointer("/info/summary")
        .ok_or(AppError::PyPISummaryError)?
        .as_str()
        .ok_or(AppError::PyPISummaryError)?;
    Ok((package, description.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_package_description() {
        let res = get_package_description("Flask".to_string());
        assert!(res.unwrap().1.contains("simple framework"));
    }

}