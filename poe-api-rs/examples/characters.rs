extern crate clap;
extern crate poe_api as poe;

use clap::{App, Arg};

fn main() {
    let matches = App::new("PoE Characters")
        .about("Path of Exile Character CLI.")
        .arg(Arg::with_name("ACCOUNT_NAME")
            .help("PoE Account Name")
            .required(true)
            .index(1))
        .get_matches();

    let account_name = matches.value_of("ACCOUNT_NAME").unwrap();

    let characters = poe::characters(account_name)
        .expect("Failed to get data.");

    println!("{:#?}", characters);
}
