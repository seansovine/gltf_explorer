//! Read info from a glTF file using the document and buffers returned
//! by the `import` function of the glTF crate (its higher-level API).

use std::cell::RefCell;

use gltf::{Document, Mesh, Node, buffer::Data, mesh::Mode, scene::Transform};

const DEFAULT_COLOR: [f32; 3] = [1.0, 0.0, 0.0];

pub struct GpuVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Default for GpuVertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 1.0],
            normal: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
        }
    }
}

#[derive(Default)]
pub struct RenderMesh {
    pub vertices: Vec<GpuVertex>,
}

#[derive(Default)]
pub struct RenderScene {
    pub meshes: Vec<RenderMesh>,
}

pub struct GltfLoader {
    pub document: Document,
    pub buffer_data: Vec<Data>,

    // Could avoid RefCell, but it simplifies function signatures for now.
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
            // Traverse root nodes of scene.
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
                    panic!(
                        "UNIMPLEMENTED: Reader expects no non-trivial decomposed transformations."
                    );
                }
            }
        }
        println!();
    }

    fn log_mesh(&self, mesh: &Mesh, depth: usize) {
        Self::indent(depth);
        println!("Node has mesh.");

        let mut render_scene = self.render_scene.borrow_mut();
        render_scene.meshes.push(RenderMesh::default());
        let render_mesh = render_scene.meshes.last_mut().unwrap();

        for primitive in mesh.primitives() {
            if primitive.mode() != Mode::Triangles {
                continue;
            }

            let reader = primitive.reader(|buff_idx| Some(&self.buffer_data[buff_idx.index()]));
            let iter = reader
                .read_positions()
                .unwrap()
                .zip(reader.read_normals().unwrap());

            for (position, normal) in iter {
                render_mesh.vertices.push(GpuVertex {
                    position,
                    color: DEFAULT_COLOR,
                    normal,
                    ..Default::default()
                });
            }
        }
    }
}
