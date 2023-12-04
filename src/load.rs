use crate::{config, Error};
use fast_image_resize::{Image, PixelType};
use std::num::NonZeroU32;
use std::path::Path;
use std::sync::Arc;

type ImageInfo = ((u32, u32), Vec<u8>, PixelType);

#[cfg(feature = "image")]
fn img_load_from_path(path: &Path) -> Result<ImageInfo, Error> {
    let img = image::io::Reader::open(path)?.decode()?;

    Ok((
        (img.width(), img.height()),
        img.to_rgba8().into_raw(),
        PixelType::U8x4,
    ))
}

#[cfg(not(feature = "image"))]
fn oxi_load_from_path(path: &Path, options: &oxipng::Options) -> Result<ImageInfo, Error> {
    // Ability to load an image is behind a private-ish module without
    // compatibility guarantees at present, also not the most efficient.
    use oxipng::internal_tests::PngData;

    let png = Arc::into_inner(PngData::new(path, options)?.raw).unwrap();

    Ok((
        (png.ihdr.width, png.ihdr.height),
        png.data,
        match png.ihdr.bpp() {
            // Not handling paletted PNG images at present
            24 => PixelType::U8x3,
            32 => PixelType::U8x4,
            n => return Err(format!("unknown bits per pixel: {}", n).into()),
        },
    ))
}

// TODO: Want a variant that takes &[u8] for loading from memory?

pub fn load_from_config<'a>(config: &config::Config) -> Result<Image<'a>, Error> {
    let ((width, height), data, pixel_type) = match &config.tileformat {
        #[cfg(not(feature = "image"))]
        config::Format::OxiPng(options) => oxi_load_from_path(config.filename, &options),
        #[cfg(feature = "image")]
        _ => img_load_from_path(config.filename),
    }?;

    Ok(Image::from_vec_u8(
        NonZeroU32::new(width).ok_or("zero width")?,
        NonZeroU32::new(height).ok_or("zero height")?,
        data,
        pixel_type,
    )?)
}
