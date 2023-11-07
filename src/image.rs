use image; // someway?




fn tile_image(config: Config) -> Result<()> {
    let img = ImageReader::open(config.filename)?.decode()?;

    let tilesize = config.tilesize;

    // Needs to be a loop, for each possible tile in the image
    for x in 0..img.width() {
        for y in 0..img.height() {
            let sub = img.sub_image(x * tilesize, 0, (x + 1) * tilesize, 256);
            sub.save("0_0_0.png")?;
        }
    }
}
