use image::io::Reader as ImageReader;
use image::GenericImage;
use crate::{Config, Error};


pub fn tile_image(config: Config) -> Result<(), Error> {
    let mut img = ImageReader::open(config.filename)?.decode()?;

    let tilesize = config.tilesize;

    // Needs to be a loop, for each possible tile in the image
    for x in 0..1 {
        for y in 0..1 {
            let sub = img.sub_image(x, y, tilesize.into(), tilesize.into());
            sub.to_image().save("0_0_0.png")?;
        }
    }
    return Ok(());
}
