mod nft;
mod setup;

pub const RARITY_DELIMITER: &str = "#";

fn main() {
    setup::build_setup();
    let result = nft::get_rarity_weight("hi#20.jpg");

    println!("{:?}", result);
}
