extern crate poe_api as poe;

fn main() {
    let data = poe::public_stash_tabs()
        .expect("Failed to get data.");

    for tab in data.stashes {
        println!("{:?}", tab);
    }
}
