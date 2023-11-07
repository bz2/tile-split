use tile_split::{Config, tile_image};

// what is main() returning?
fn main() {
    println!("Hello, world!");

    // read $1
    // read envvars
    // make Config
    let config = Config { tilesize: 256 };

    tile_image(config);
    // return result code?
}
