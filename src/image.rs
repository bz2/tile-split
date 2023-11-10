use crate::{Config, Error};
use fast_image_resize::{CropBox, Image, PixelType, Resizer, ResizeAlg};
use oxipng::internal_tests::{PngData};
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU32;

fn save_view(img: &Image, x: u32, y: u32, config: &Config) -> Result<(), Error> {
    let x1 = x * config.tilesize;
    let y1 = y * config.tilesize;
    let mut view = img.view();
    let size = NonZeroU32::new(config.tilesize).ok_or("zero size")?;
    view.set_crop_box(CropBox {left: x1, top: y1, width: size, height: size})?;
    let mut sub = Image::new(size, size, view.pixel_type());
    let mut resizer = Resizer::new(ResizeAlg::default());
    resizer.resize(&view, &mut sub.view_mut())?;
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
        match view.pixel_type() {
            PixelType::U8x3 => oxipng::ColorType::RGB { transparent_color: None },
            PixelType::U8x4 => oxipng::ColorType::RGBA,
            t => return Err(format!("unknown pixel type: {:?}", t).into()),
        },
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

    let max_x = png.ihdr.height / config.tilesize;
    let max_y = png.ihdr.width / config.tilesize;
    let mut coords: Vec<(u32, u32)> = Vec::with_capacity((max_x * max_y) as usize);
    for y in 0..max_y {
        for x in 0..max_x {
            coords.push((x, y));
        }
    }
    coords.par_iter().try_for_each(|&(x, y)| save_view(&img, x, y, &config))?;
    return Ok(());
}
