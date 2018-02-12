#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub name: String,
    pub league: String,
    pub class_id: i32,
    pub class: String,
    pub level: i32,
}

#[derive(Deserialize, Debug)]
pub struct CharacterWindowGetItems {
    pub items: Vec<Item>,
    pub character: Character,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    ilvl: i32,
    name: String,
    type_line: String,
    implicit_mods: Option<Vec<String>>,
    explicit_mods: Option<Vec<String>>,
    inventory_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: String,
    pub description: String,
    pub url: Option<String>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PublicStashTab {
    pub next_change_id: String,
    pub stashes: Vec<StashTab>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StashTab {
    pub id: String,
    pub public: bool,
    pub account_name: Option<String>,
    pub last_character_name: Option<String>,
    pub stash: Option<String>,
    pub stash_type: String,
}
