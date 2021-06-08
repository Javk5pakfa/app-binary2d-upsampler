use anyhow::Result;
use binary2d::*;
use clap::{App as clapApp, Arg, ArgMatches};
use binary2d::app::{AnyState, AnyHydro};
use binary2d::traits::{Conserved};
use binary2d::state::{State, BlockState};
use ndarray::{ArcArray, Ix2, s};
// use glob::glob;


fn argument_parse(default_name: &str) -> ArgMatches {

    let matches = clapApp::new("Checkpoint Up-sampling Program")
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


fn _convert_state<C: Conserved>(_state_to_convert: State<C>) -> State<C> {
    unimplemented!()
}


fn upsample_isothermal(state_to_mod: State<hydro_iso2d::Conserved>, file_to_mod: &app::App) -> State<hydro_iso2d::Conserved> {

    // Gather info from the checkpoint state.
    let mut old_solutions = state_to_mod.solution.clone();

    match &file_to_mod.config.hydro {
        AnyHydro::Isothermal(this_hydro) => {
            for (index_pair, old_block) in old_solutions.clone() {
                let mut new_block_state: BlockState<hydro_iso2d::Conserved> = state::BlockState::from_model(
                    &file_to_mod.config.model, this_hydro, &file_to_mod.config.mesh, index_pair
                ).unwrap().into();
                let old_conserved = old_block.conserved.to_owned();
                // new_block_state.conserved.slice_mut(s![0..2, 0..2]) = old_conserved;
            }
        }
        AnyHydro::Euler(_this_hydro) => (),
    }
    state_to_mod
}



fn upsample_isothermal_v2(app: app::App) -> app::App {

    let state = match &app.state {
        AnyState::Isothermal(this_state) => this_state,
        AnyState::Euler(this_state) => panic!(),
    };

    // Upsample...
    for (index, block) in &state.solution {

        let new_conserved = ndarray::Array::<_, Ix2>::from_shape_fn((0, 0), |(i, j)| hydro_iso2d::Conserved(0.0, 0.0, 0.0));
        let new_block = state::BlockState{
            conserved: new_conserved.to_shared(),
            integrated_source_terms: block.integrated_source_terms,
            orbital_elements_change: block.orbital_elements_change
        };
    }

    app
}




fn _upsample_euler<C: Conserved>(_state_to_mod: State<C>) -> State<C> {
    unimplemented!()
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
    // let old_bs = file.config.mesh.block_size;
    // let nb = file.config.mesh.num_blocks;
    // let new_bs = old_bs * 2;
    //
    // // Print useful info.
    // println!("Checkpoint file provided is ...{}", f);
    // println!("Output file will be named .....{}", out_f);
    // println!("Number of blocks is ...........{}", nb);
    // println!("Old block size is .............{}", old_bs);
    // println!("New block size will be ........{}", new_bs);

    // Update the new file.

    // Update file mesh info.
    // file.config.mesh.num_blocks = nb;
    //
    // let state = file.state.clone();
    // let new_state = match state {
    //     AnyState::Isothermal(state) => AnyState::Isothermal(upsample_isothermal(state, &file)),
    //     AnyState::Euler(_state) => anyhow::bail!("Not yet implemented!"),
    // };
    // file.state = new_state;

    let new_file = upsample_isothermal_v2(file);

    // Write updated checkpoint file to directory.
    // new_file.state = new_state;
    io::write_cbor(&new_file, out_f)?;

    Ok(())
}
