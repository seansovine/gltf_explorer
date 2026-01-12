//! Read info from a glTF file using the document and buffers returned
//! by the `import` function of the glTF crate (its higher-level API).

use std::cell::RefCell;

use gltf::{Document, Mesh, Node, buffer::Data, mesh::Mode, scene::Transform};

#[derive(Default)]
pub struct RenderMesh {}

#[derive(Default)]
pub struct RenderScene {
    pub meshes: Vec<RenderMesh>,
}

pub struct GltfLoader {
    pub document: Document,
    pub buffer_data: Vec<Data>,
    pub render_scene: RefCell<RenderScene>,
}

impl GltfLoader {
    pub fn create(gltf_path: &str) -> Self {
        let (document, buffer_data, _images) = gltf::import(gltf_path).unwrap();
        Self {
            document,
            buffer_data,
            render_scene: Default::default(),
        }
    }
}

impl GltfLoader {
    pub fn traverse(&self) {
        for scene in self.document.scenes() {
            for node in scene.nodes() {
                self.log_node(&node, 1);
                self.traverse_children(&node, 2);
            }
        }
        println!(
            "# of meshes found: {}",
            self.render_scene.borrow().meshes.len()
        );
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

        let mut render_scene = self.render_scene.borrow_mut();
        render_scene.meshes.push(RenderMesh {});

        for primitive in mesh.primitives() {
            if primitive.mode() != Mode::Triangles {
                continue;
            }

            let reader = primitive.reader(|buff_idx| Some(&self.buffer_data[buff_idx.index()]));
            let _position_reader = reader.read_positions().unwrap();

            // TODO: Read data into the structure.
        }
    }
}
