use anyhow::Result;
use clap::{App as clapApp, Arg};
use binary2d::*;


fn main() -> Result<()> {

    let default_file_name = "upsampled.cbor";

    let matches = clapApp::new("Upsample Config")
        .author("Zhongtian H. <zhongth@clemson.edu>")
        .arg(Arg::with_name("INPUT")
            .help("Input file name")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .help("Output file name")
            .default_value(default_file_name)
            .index(2))
        .get_matches();

    let f = matches.value_of("INPUT").unwrap();
    let out_f = matches.value_of("OUTPUT").unwrap();

    let file: app::App = binary2d::io::read_cbor(f)?;
    io::write_cbor(&file, out_f)?;

    Ok(())
}
