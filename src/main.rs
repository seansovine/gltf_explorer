//! Using the gltf crate to explore the gltf file format.

use std::error::Error;

use clap::{Arg, ArgAction, Command};
use gltf::Gltf;

const TEST_FILE_PATH: &str =
    "/home/sean/Code_projects/wgpu_grapher/scratch/ferrari_monza/scene.gltf";
// Other file: "/home/sean/Code_open_source/graphics/models/glTF-Sample-Models/2.0/AntiqueCamera/glTF-Binary/AntiqueCamera.glb";

fn main() -> Result<(), Box<dyn Error>> {
    let app_args = Command::new("glTF Explorer")
        .arg(Arg::new("traverse").short('t').action(ArgAction::SetTrue))
        .arg(Arg::new("convert").short('c').action(ArgAction::SetTrue))
        .get_matches();

    let file_data = std::fs::read(TEST_FILE_PATH)?;
    let gltf = Gltf::from_slice(&file_data)?;

    if app_args.get_flag("convert") {
        gltf_explorer::convert(&gltf)?;
    }
    if app_args.get_flag("traverse") {
        gltf_explorer::traverse(&gltf);
    }

    Ok(())
}
