use crate::{Config, Error};
use fast_image_resize::{change_type_of_pixel_components_dyn, CropBox, Image, PixelType};
use oxipng::internal_tests::{PngData};
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU32;

fn save_view(img: &Image, x: u32, y: u32, config: &Config) -> Result<(), Error> {
    let x1 = x * config.tilesize;
    let y1 = y * config.tilesize;
    let mut view = img.view();
    let size = NonZeroU32::new(config.tilesize).ok_or("zero size")?;
    view.set_crop_box(CropBox {left: x1, top: y1, width: size, height: size})?;
    let mut sub = Image::new(size, size, PixelType::U8x4);
    change_type_of_pixel_components_dyn(&view, &mut sub.view_mut())?;
    let path = format!(
        "{p}/{z}-{x}-{y}.png",
        p = config.folder,
        z = config.zoomlevel,
        x = x,
        y = y
    );
    let png = oxipng::RawImage::new(
        config.tilesize,
        config.tilesize,
        oxipng::ColorType::RGBA,
        oxipng::BitDepth::Eight,
        sub.into_vec(),
    )?
    .create_optimized_png(&oxipng::Options {
        deflate: oxipng::Deflaters::Libdeflater { compression: 10 },
        ..Default::default()
    })?;
    let mut file = File::create(path)?;
    file.write_all(&png)?;

    Ok(())
}

pub fn tile_image(config: Config) -> Result<(), Error> {
    let png = std::sync::Arc::into_inner(PngData::new(std::path::Path::new(config.filename), &oxipng::Options::default() )?.raw).unwrap();

    let img = fast_image_resize::Image::from_vec_u8(
        NonZeroU32::new(png.ihdr.width).ok_or("zero width")?,
        NonZeroU32::new(png.ihdr.height).ok_or("zero height")?,
        png.data,
        match png.ihdr.bpp() {
            24 => PixelType::U8x3,
            32 => PixelType::U8x4,
            n => return Err(format!("unknown bits per pixel: {}", n).into()),
        }
    )?;

    // Needs to be a loop, for each possible tile in the image
    for y in 0..(png.ihdr.height / config.tilesize) {
        for x in 0..(png.ihdr.width / config.tilesize) {
            save_view(&img, x, y, &config)?;
        }
    }
    return Ok(());
}
