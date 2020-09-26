use error::AppError;

mod error;
mod pypi;

pub struct Item {
    pub name: String,
    pub link: String,
    pub description: String,
}

pub fn run() -> Result<(), AppError> {
    let pypi_items = pypi::process()?;

    Ok(())
}
