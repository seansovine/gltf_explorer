//! Using the gltf crate to explore the gltf file format.

use std::error::Error;

use gltf::{Glb, Gltf};

const TEST_FILE_PATH: &str = "/home/sean/Code_open_source/graphics/models/glTF-Sample-Models/2.0/AntiqueCamera/glTF-Binary/AntiqueCamera.glb";

fn main() -> Result<(), Box<dyn Error>> {
    let file_data = std::fs::read(TEST_FILE_PATH)?;

    // Not particularly useful.
    if false {
        let glb = Glb::from_slice(&file_data)?;
        println!("JSON data:\n{:?}", glb.json);
    }

    let gltf = Gltf::from_slice(&file_data)?;
    for scene in gltf.scenes() {
        for node in scene.nodes() {
            println!(
                "Node #{} has {} children",
                node.index(),
                node.children().count(),
            );
        }
    }

    // Write gltf json data to file.
    let json = gltf.document.as_json().to_string_pretty()?;
    println!("{json}");

    Ok(())
}
