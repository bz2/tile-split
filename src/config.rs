use std::ops::RangeInclusive;
use std::path::Path;
use std::str::FromStr;

use crate::Error;

#[derive(Clone, Debug)]
pub enum Format {
    OxiPng(oxipng::Options),
    #[cfg(feature = "image")]
    Image(image::ImageFormat),
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "o0" | "o1" | "o2" | "o3" | "o4" | "o5" => Some(Format::OxiPng(
                oxipng::Options::from_preset(input[1..].parse().unwrap()),
            )),
            #[cfg(feature = "image")]
            _ => image::ImageFormat::from_extension(input).map(Format::Image),
            #[cfg(not(feature = "image"))]
            _ => None,
        }
        .ok_or("unknown format".into())
    }
}

pub struct Config<'a> {
    pub filename: &'a Path,                       // $1
    pub tilesize: u32,                            // 256
    pub zoomlevel: u8,                            // eg 5
    pub zoomrange: RangeInclusive<u8>,            // eg 0 - 5
    pub targetrange: Option<RangeInclusive<u32>>, //eg 0 - 500
    pub tileformat: Format,                       // png
    pub folder: &'a Path,                         // $OUTDIR out
}

impl<'a> Config<'a> {
    /// Give a string suitable for use as an output image file extention
    pub fn extension(&self) -> &'static str {
        match self.tileformat {
            #[cfg(feature = "image")]
            Format::Image(img) => img.extensions_str()[0],
            _ => "png",
        }
    }
}
