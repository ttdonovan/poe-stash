use reqwest;

use std::error;

pub fn get(uri: &str) -> Result<String, Box<error::Error>> {
    let mut resp = reqwest::get(uri)?;
    let body = resp.text()?;
    Ok(body)
}
