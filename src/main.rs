use tile_split::{tile_image, Config};

// what is main() returning?
fn main() {
    // read $1
    let mut args = std::env::args();
    args.next().unwrap();
    let filename = &args.next().unwrap();
    // read envvars
    // make Config
    let config = Config {
        tilesize: 256,
        filename,
        zoomlevel: 5,
        folder: "out",
    };

    std::fs::create_dir_all(config.folder).unwrap();

    tile_image(config).unwrap();
    // return result code?
}
