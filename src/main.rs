//! Using the gltf crate to explore the gltf file format.

use std::error::Error;

use clap::{Arg, ArgAction, Command};
use gltf::Gltf;

use gltf_explorer::{high_level, low_level};

const TEST_FILE_PATH: &str =
    "/home/sean/Code_projects/wgpu_grapher/scratch/gltfs/ferrari_monza/scene.gltf";
// Other file: "/home/sean/Code_open_source/graphics/models/glTF-Sample-Models/2.0/AntiqueCamera/glTF-Binary/AntiqueCamera.glb";

const READ_LOW_LEVEL: bool = false;

fn main() -> Result<(), Box<dyn Error>> {
    let app_args = Command::new("glTF Explorer")
        .arg(Arg::new("traverse").short('t').action(ArgAction::SetTrue))
        .arg(Arg::new("convert").short('c').action(ArgAction::SetTrue))
        .get_matches();

    if app_args.get_flag("convert") {
        let file_data = std::fs::read(TEST_FILE_PATH)?;
        let gltf = Gltf::from_slice(&file_data)?;
        gltf_explorer::convert(&gltf)?;
    }
    if app_args.get_flag("traverse") {
        if READ_LOW_LEVEL {
            let file_data = std::fs::read(TEST_FILE_PATH)?;
            let gltf = Gltf::from_slice(&file_data)?;
            let loader = low_level::GltfLoader::create(&gltf);
            loader.traverse();
        } else {
            let loader = high_level::GltfLoader::create(TEST_FILE_PATH);
            loader.traverse();
        }
    }

    Ok(())
}
