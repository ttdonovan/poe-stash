extern crate poe_api as poe;

fn main() {
    let data = poe::leagues()
        .expect("Failed to get data.");

    for league in data {
        println!("{:?}", league);
    }
}
