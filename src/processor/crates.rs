use crate::error::AppError;
use crate::Item;
use isahc::ResponseExt;
use serde_json::Value;

const API_URL: &str = "https://crates.io/api/v1/crates/${package}";
const LIST_FILE: &str = "data/crate_list.txt";
const DESCRIPTION_FILE: &str = "data/crate_description.toml";
const GROUP_FILE: &str = "data/crate.toml";

pub fn process(link: &str) -> Result<Vec<Item>, AppError> {
    let packages = super::process_list_file(LIST_FILE)?;
    super::process_description_file(packages, DESCRIPTION_FILE, get_package_description)?;
    super::process_description_group(link, DESCRIPTION_FILE, GROUP_FILE)
}

pub fn get_package_description(package: String) -> Result<(String, String), AppError> {
    let url = API_URL.replace("${package}", &package);
    let mut res = isahc::get(url)?;
    let res: Value = serde_json::from_str(res.text()?.as_str())?;
    let description = res
        .pointer("/crate/description")
        .ok_or(AppError::PyPISummaryError)?
        .as_str()
        .ok_or(AppError::PyPISummaryError)?
        .trim().replace("\\n", "");
    Ok((package, description.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_package_description() {
        let res = get_package_description("url".to_string());
        dbg!(&res);
        assert!(res.unwrap().1.contains("library for Rust"));
    }
}
