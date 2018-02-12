extern crate clap;
extern crate poe_api as poe;

use clap::{App, Arg};

fn main() {
    let matches = App::new("PoE Character Window")
        .about("Path of Exile Character Window CLI.")
        .arg(Arg::with_name("ACCOUNT_NAME")
            .help("PoE Account Name")
            .required(true)
            .index(1))
        .arg(Arg::with_name("CHARACTER")
            .help("PoE Character Name")
            .required(true)
            .index(2))
        .arg(Arg::with_name("DETAIL")
            .help("PoE Character Detail - items, passives or stash")
            .required(true)
            .index(3))
        .get_matches();

    let account_name = matches.value_of("ACCOUNT_NAME").unwrap();
    let characeter = matches.value_of("CHARACTER").unwrap();
    let detail = matches.value_of("DETAIL").unwrap();

    match detail {
        "items" => {
            let items = poe::character_and_items(account_name, characeter)
                .expect("Failed to get data.");

            println!("{:#?}", items);
        },
        "passives" => {
            // let passives = poe::character_and_passives(account_name, characeter)
            //     .expect("Failed to get data.");
            //
            // println!("{:#?}", passives);
        },
        "stash" => {
            // let stash = poe::character_and_stash(account_name, characeter)
            //     .expect("Failed to get data.");
            //
            // println!("{:#?}", stash);
        },
        _ => {
            println!("Sorry but cannot help with that request.")
        }
    }
}
