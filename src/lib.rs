pub mod high_level;
pub mod low_level;
pub mod render_data;

use std::error::Error;

use gltf::Gltf;

/// Extract JSON data and print as formatted JSON.
/// Useful for extracting glTF JSON from glTF binary.
pub fn convert(gltf: &Gltf) -> Result<(), Box<dyn Error>> {
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
