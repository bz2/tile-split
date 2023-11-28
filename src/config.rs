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
    pub filename: &'a Path, // $1
    pub tilesize: u32,      // 256
    pub zoomlevel: u8,      // eg 5
    pub startzoomrangetoslice: u8,
    pub endzoomrangetoslice: u8,
    pub starttargetrange: u32,
    pub endtargetrange: u32,
    pub tileformat: Format,                       // png
}

impl<'a> Config<'a> {
    pub fn new(
        filename: &'a Path,                       // $1
        tilesize: u32,                            // 256
        zoomlevel: u8,                            // eg 5
        zoomrange: Option<RangeInclusive<u8>>,    // eg 0 - 5
        targetrange: Option<RangeInclusive<u32>>, //eg 0 - 500
    ) -> Self {
        let zomr = zoomrange.unwrap_or(zoomlevel..=zoomlevel);
        // total number of tiles required in zoomrange
        let mut totaltiles = 0;
        zomr.clone().for_each(|x| {
            totaltiles += 1 << (x * 2);
        });
        // number of tiles sliced
        let tilessliced = match &targetrange {
            Some(targetrange) => *targetrange.start(),
            None => 0,
        };
        // number of tiles to slice
        let tilestoslice = match &targetrange {
            Some(targetrange) => *targetrange.end() - tilessliced,
            None => totaltiles,
        };
        if tilestoslice > totaltiles {
            panic!("Target range value cannot be over than the total number of tiles within zoom range.");
        }

        // zoom level to start
        let mut startzoomrangetoslice: u8 = *zomr.clone().start();
        // tile index to start
        let mut starttargetrange: u32 = 0;
        // zoom level to stop
        let mut endzoomrangetoslice: u8 = *zomr.clone().end();
        // tile index to stop
        let mut endtargetrange: u32 = 0;

        // total of tiles in previous zoom levels
        let mut tilessum = 0;
        // calculte startzoomrangetoslice and starttargetrange
        for i in zomr.clone() {
            // number of tiles in this zoom level
            let currentzoomtiles = 1 << (i * 2);
            if tilessum + currentzoomtiles > tilessliced {
                startzoomrangetoslice = i;
                starttargetrange = tilessliced - tilessum;
                break;
            } else {
                tilessum += currentzoomtiles;
            }
        }
        tilessum = 0;
        // calculte endzoomrangetoslice and endtargetrange
        for i in zomr.clone() {
            // number of tiles in this zoom level
            let currentzoomtiles = 1 << (i * 2);
            if tilessum + currentzoomtiles >= tilessliced + tilestoslice {
                endzoomrangetoslice = i;
                endtargetrange = tilessliced + tilestoslice - tilessum - 1;
                break;
            } else {
                tilessum += currentzoomtiles;
            }
        }

        Config {
            tilesize,
            filename,
            zoomlevel,
            startzoomrangetoslice,
            endzoomrangetoslice,
            starttargetrange,
            endtargetrange,
            // TODO: tileformat
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;
    use std::{ops::RangeInclusive, path::Path};

    #[test]
    // build config with only required args
    fn minimum_args() {
        let config = Config::new(&Path::new("test.png"), 256, 5, None, None);
        assert_eq!(config.startzoomrangetoslice, 5);
        assert_eq!(config.starttargetrange, 0);
        assert_eq!(config.endzoomrangetoslice, 5);
        assert_eq!(config.endtargetrange, 1023);
    }

    #[test]
    // slice all tiles
    fn full_zoom() {
        let config = Config::new(
            &Path::new("test.png"),
            256,
            5,
            Some(RangeInclusive::new(0, 5)),
            None,
        );
        assert_eq!(config.startzoomrangetoslice, 0);
        assert_eq!(config.starttargetrange, 0);
        assert_eq!(config.endzoomrangetoslice, 5);
        assert_eq!(config.endtargetrange, 1023);
    }

    #[test]
    // slice the first 341 tiles out of all tiles
    fn full_zoom_1() {
        let config = Config::new(
            &Path::new("test.png"),
            256,
            5,
            Some(RangeInclusive::new(0, 5)),
            Some(RangeInclusive::new(0, 341)),
        );
        assert_eq!(config.startzoomrangetoslice, 0);
        assert_eq!(config.starttargetrange, 0);
        assert_eq!(config.endzoomrangetoslice, 4);
        assert_eq!(config.endtargetrange, 255);
    }

    #[test]
    // slice the second 341 tiles out of all tiles
    fn full_zoom_2() {
        let config = Config::new(
            &Path::new("test.png"),
            256,
            5,
            Some(RangeInclusive::new(0, 5)),
            Some(RangeInclusive::new(341, 682)),
        );
        assert_eq!(config.startzoomrangetoslice, 5);
        assert_eq!(config.starttargetrange, 0);
        assert_eq!(config.endzoomrangetoslice, 5);
        assert_eq!(config.endtargetrange, 340);
    }

    #[test]
    // slice the third 341 tiles out of all tiles
    fn full_zoom_3() {
        let config = Config::new(
            &Path::new("test.png"),
            256,
            5,
            Some(RangeInclusive::new(0, 5)),
            Some(RangeInclusive::new(682, 1023)),
        );
        assert_eq!(config.startzoomrangetoslice, 5);
        assert_eq!(config.starttargetrange, 341);
        assert_eq!(config.endzoomrangetoslice, 5);
        assert_eq!(config.endtargetrange, 681);
    }

    #[test]
    // slice the remaining tiles out of all tiles
    fn full_zoom_4() {
        let config = Config::new(
            &Path::new("test.png"),
            256,
            5,
            Some(RangeInclusive::new(0, 5)),
            Some(RangeInclusive::new(1023, 1365)),
        );
        assert_eq!(config.startzoomrangetoslice, 5);
        assert_eq!(config.starttargetrange, 682);
        assert_eq!(config.endzoomrangetoslice, 5);
        assert_eq!(config.endtargetrange, 1023);
    }

    #[test]
    // slice the first 448 tiles out of all tiles
    fn half_zoom_1() {
        let config = Config::new(
            &Path::new("test.png"),
            256,
            5,
            Some(RangeInclusive::new(3, 5)),
            Some(RangeInclusive::new(0, 448)),
        );
        assert_eq!(config.startzoomrangetoslice, 3);
        assert_eq!(config.starttargetrange, 0);
        assert_eq!(config.endzoomrangetoslice, 5);
        assert_eq!(config.endtargetrange, 127);
    }

    #[test]
    // slice the second 448 tiles out of all tiles
    fn half_zoom_2() {
        let config = Config::new(
            &Path::new("test.png"),
            256,
            5,
            Some(RangeInclusive::new(3, 5)),
            Some(RangeInclusive::new(448, 896)),
        );
        assert_eq!(config.startzoomrangetoslice, 5);
        assert_eq!(config.starttargetrange, 128);
        assert_eq!(config.endzoomrangetoslice, 5);
        assert_eq!(config.endtargetrange, 575);
    }

    #[test]
    // slice the remaining tiles out of all tiles
    fn half_zoom_3() {
        let config = Config::new(
            &Path::new("test.png"),
            256,
            5,
            Some(RangeInclusive::new(3, 5)),
            Some(RangeInclusive::new(896, 1344)),
        );
        assert_eq!(config.startzoomrangetoslice, 5);
        assert_eq!(config.starttargetrange, 576);
        assert_eq!(config.endzoomrangetoslice, 5);
        assert_eq!(config.endtargetrange, 1023);
    }
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
