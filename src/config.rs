
pub struct Config<'a> {
    pub filename: &'a std::path::Path, // $1
    pub tilesize: u32, // 256
    pub zoomlevel: u8, // eg 0 - 5
    pub tileformat: &'a str, // png
    pub folder: &'a std::path::Path, // $OUTDIR out
}
