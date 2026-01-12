//! Gets information about the glTF file using the gltf crate low-level API.

use gltf::{Buffer, Gltf, Mesh, Node, Scene, mesh::Mode, scene::Transform};

pub struct GltfLoader<'a> {
    pub scenes: Vec<Scene<'a>>,
    pub buffers: Vec<Buffer<'a>>,
}

impl<'a> GltfLoader<'a> {
    pub fn create(gltf: &'a Gltf) -> GltfLoader<'a> {
        Self {
            scenes: gltf.scenes().collect(),
            buffers: gltf.buffers().collect(),
        }
    }
}

impl<'a> GltfLoader<'a> {
    /// Traverse glTF scene + node tree and print information about nodes.
    pub fn traverse(&self) {
        for scene in &self.scenes {
            println!(
                "Scene {}: {}",
                scene.index(),
                scene.name().unwrap_or("<UNNAMED>")
            );
            for node in scene.nodes() {
                self.log_node(&node, 1);
                self.traverse_children(&node, 2);
            }
        }
    }

    fn traverse_children(&self, node: &Node, depth: usize) {
        for child in node.children() {
            self.log_node(&child, depth);
            self.traverse_children(&child, depth + 1);
        }
    }

    fn indent(depth: usize) {
        const INDENT: usize = 4;
        print!("{}", " ".repeat(depth * INDENT));
    }

    fn log_node(&self, node: &Node, depth: usize) {
        Self::indent(depth);
        println!(
            "Node {}: {}",
            node.index(),
            node.name().unwrap_or("<UNNAMED>")
        );
        if let Some(mesh) = node.mesh() {
            self.log_mesh(&mesh, depth + 1);
        }
        match node.transform() {
            Transform::Matrix { .. } => {
                Self::indent(depth + 1);
                println!("Node has matrix.");
            }
            Transform::Decomposed {
                translation,
                rotation,
                scale,
            } => {
                Self::indent(depth + 1);
                println!("Node has decomposed transformation.");
                let nontrivial = translation != [0.0_f32, 0.0_f32, 0.0_f32]
                    || rotation != [0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32]
                    || scale != [1.0_f32, 1.0_f32, 1.0_f32];
                if nontrivial {
                    Self::indent(depth + 2);
                    println!("Nontrivial translation: {translation:?}");
                    Self::indent(depth + 2);
                    println!("Rotation: {rotation:?}");
                    Self::indent(depth + 2);
                    println!("Scale: {scale:?}");
                }
            }
        }
        println!();
    }

    fn log_mesh(&self, mesh: &Mesh, depth: usize) {
        Self::indent(depth);
        println!("Node has mesh.");

        for primitive in mesh.primitives() {
            if primitive.mode() != Mode::Triangles {
                continue;
            }

            // TODO.
        }
    }
}
