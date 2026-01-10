use std::error::Error;

use gltf::{Gltf, Node, scene::Transform};

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

/// Traverse glTF scene + node tree and print information about nodes.
pub fn traverse(gltf: &Gltf) {
    for scene in gltf.scenes() {
        println!(
            "Scene {}: {}",
            scene.index(),
            scene.name().unwrap_or("<UNNAMED>")
        );
        for node in scene.nodes() {
            log_node(&node, 1);
            traverse_children(&node, 2);
        }
    }
}

fn traverse_children(node: &Node, depth: usize) {
    for child in node.children() {
        log_node(&child, depth);
        traverse_children(&child, depth + 1);
    }
}

fn indent(depth: usize) {
    const INDENT: usize = 4;
    print!("{}", " ".repeat(depth * INDENT));
}

fn log_node(node: &Node, depth: usize) {
    indent(depth);
    println!(
        "Node {}: {}",
        node.index(),
        node.name().unwrap_or("<UNNAMED>")
    );
    if node.mesh().is_some() {
        indent(depth + 1);
        println!("Node has mesh.")
    }
    match node.transform() {
        Transform::Matrix { .. } => {
            indent(depth + 1);
            println!("Node has matrix.");
        }
        Transform::Decomposed {
            translation,
            rotation,
            scale,
        } => {
            indent(depth + 1);
            println!("Node has decomposed transformation.");
            let nontrivial = translation != [0.0_f32, 0.0_f32, 0.0_f32]
                || rotation != [0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32]
                || scale != [1.0_f32, 1.0_f32, 1.0_f32];
            if nontrivial {
                indent(depth + 2);
                println!("Nontrivial translation: {translation:?}");
                indent(depth + 2);
                println!("Rotation: {rotation:?}");
                indent(depth + 2);
                println!("Scale: {scale:?}");
            }
        }
    }
    println!();
}
