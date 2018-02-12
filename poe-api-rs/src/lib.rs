extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::error;

const API_ENDPOINT: &str = "http://api.pathofexile.com";
const WWW_ENDPOINT: &str = "https://www.pathofexile.com";

pub mod models;
mod provider;

use models::*;

pub fn leagues() -> Result<Vec<League>, Box<error::Error>> {
    let uri = format!("{}/leagues", API_ENDPOINT);
    let res = provider::get(&uri)?;
    let data : Vec<League> = serde_json::from_str(&res)?;
    Ok(data)
}

pub fn public_stash_tabs() -> Result<PublicStashTab, Box<error::Error>> {
    let uri = format!("{}/public-stash-tabs", API_ENDPOINT);
    let res = provider::get(&uri)?;
    let data : PublicStashTab = serde_json::from_str(&res)?;
    Ok(data)
}

pub fn characters(account_name: &str) -> Result<Vec<Character>, Box<error::Error>> {
    let uri = format!("{}/character-window/get-characters?accountName={}", WWW_ENDPOINT, account_name);
    let res = provider::get(&uri)?;
    let data : Vec<Character> = serde_json::from_str(&res)?;
    Ok(data)
}

pub fn character_and_items(account_name: &str, character: &str) -> Result<CharacterWindowGetItems, Box<error::Error>> {
    let uri = format!("{}/character-window/get-items?accountName={}&character={}", WWW_ENDPOINT, account_name, character);
    let res = provider::get(&uri)?;
    let data : CharacterWindowGetItems = serde_json::from_str(&res)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
