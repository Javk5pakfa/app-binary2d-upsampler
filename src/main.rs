use anyhow::Result;
use binary2d::*;
use clap::{App as clapApp, Arg, ArgMatches};
// use glob::glob;


fn argument_parse(default_name: &str) -> ArgMatches {

    let matches = clapApp::new("Upsample Config")
        .author("Zhongtian H. <zhongth@clemson.edu>")
        .arg(Arg::with_name("INPUT")
            .help("Input file name")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .help("Output file name")
            .default_value(default_name)
            .index(2))
        .get_matches();

    matches
}


fn main() -> Result<()> {

    let default_file_name = "chkpt.upsampled.cbor";
    let matches = argument_parse(default_file_name);

    let f = matches.value_of("INPUT").unwrap();
    let out_f = matches.value_of("OUTPUT").unwrap();
    // let old_file_path = glob(&String::from(f))?;
    // let new_file_path = glob(&String::from(out_f))?;

    // Gather data from old checkpoint file.
    let file: app::App = binary2d::io::read_cbor(f)?;
    let mut new_file = file.clone();
    let old_bs = file.config.mesh.block_size;
    let nb = file.config.mesh.num_blocks;
    let new_bs = old_bs * 2;

    // Print useful info.
    println!("Checkpoint file provided is {}", f);
    println!("Output file will be named {}", out_f);
    println!("Number of blocks is {}", nb);
    println!("Old block size is {}", old_bs);
    println!("New block size will be {}", new_bs);

    // Update the new file.
    new_file.config.mesh.block_size = new_bs;

    // Write updated checkpoint file to directory.
    io::write_cbor(&new_file, out_f)?;

    Ok(())
}
