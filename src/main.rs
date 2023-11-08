use std::env;
use tile_split::{Config, tile_image};

// what is main() returning?
fn main() {
    let filename: String = env::args().collect::<Vec<String>>()[0].clone();
    let folder: String = env::var("OUTDIR").unwrap();
    let format: String = env::var("FMT").unwrap();
    let zoomlevel: u8 = env::var("ZOOMLEVEL").unwrap().parse::<u8>().unwrap();

    let config: Config = Config {
        filename: filename.to_string(),
        folder,
        format,
        tilesize: 256,
        zoomlevel,
    };

    tile_image(config).unwrap();
    // return result code?
}
