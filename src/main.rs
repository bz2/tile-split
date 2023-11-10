use std::env;
use tile_split::{Config, tile_image};

// what is main() returning?
fn main() {
    // Exit with an error if there is no filename arg
    let filename: String = match env::args().nth(1) {
        Some(arg) => arg.to_string(),
        None => {
            eprintln!("Error: Please provide a filename.");
            std::process::exit(1);
        }
    };
    let folder: String = env::var("OUTDIR").unwrap_or("out".to_string());
    let format: String = env::var("FMT").unwrap_or("png".to_string());
    let zoomlevel: u8 = env::var("ZOOMLEVEL").map(|s| s.parse::<u8>()).unwrap_or(Ok(5)).unwrap_or(5);

    let config: Config = Config {
        filename,
        folder,
        format,
        tilesize: 256,
        zoomlevel,
    };

    tile_image(config).unwrap();
    // return result code?
}
