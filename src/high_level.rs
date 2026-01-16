//! Read info from a glTF file using the document and buffers returned
//! by the `import` function of the glTF crate (its higher-level API).

use std::cell::RefCell;

use gltf::{Document, Mesh, Node, buffer::Data, image::Source, mesh::Mode, scene::Transform};

use crate::render_data::{self, GpuVertex, MatrixUniform, RenderMesh, RenderScene};

fn node_matrix(node: &Node) -> MatrixUniform {
    match node.transform() {
        Transform::Matrix { matrix } => matrix.into(),
        // For our current models decomposed form is always identity.
        Transform::Decomposed { .. } => MatrixUniform::identity(),
    }
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
        let root_matrix = MatrixUniform::identity();
        for scene in self.document.scenes() {
            // Traverse root nodes of scene.
            for node in scene.nodes() {
                let matrix = node_matrix(&node) * root_matrix;
                self.add_node(&node, 1, &matrix);
                self.traverse_children(&node, 2, &matrix);
            }
        }
        println!(
            "# of meshes found: {}",
            self.render_scene.borrow().meshes.len()
        );
    }

    fn traverse_children(&self, node: &Node, depth: usize, parent_matrix: &MatrixUniform) {
        for child in node.children() {
            let matrix = node_matrix(node) * *parent_matrix;
            self.add_node(&child, depth, &matrix);
            self.traverse_children(&child, depth + 1, &matrix);
        }
    }

    fn indent(depth: usize) {
        const INDENT: usize = 4;
        print!("{}", " ".repeat(depth * INDENT));
    }

    fn add_node(&self, node: &Node, depth: usize, matrix: &MatrixUniform) {
        // Some logging.
        Self::log_node(node, depth);
        if let Some(mesh) = node.mesh() {
            self.add_mesh(&mesh, depth + 1, matrix);
        }
    }

    fn log_node(node: &Node, depth: usize) {
        Self::indent(depth);
        println!(
            "Node {}: {}",
            node.index(),
            node.name().unwrap_or("<UNNAMED>")
        );
        match node.transform() {
            Transform::Matrix { matrix: _ } => {
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

    fn add_mesh(&self, mesh: &Mesh, depth: usize, matrix: &MatrixUniform) {
        Self::indent(depth);
        println!("Node has mesh.");

        let mut render_scene = self.render_scene.borrow_mut();
        render_scene.meshes.push(RenderMesh {
            matrix: *matrix,
            ..Default::default()
        });
        let render_mesh = render_scene.meshes.last_mut().unwrap();

        for primitive in mesh.primitives() {
            if primitive.mode() != Mode::Triangles {
                continue;
            }
            let reader = primitive.reader(|buff_idx| Some(&self.buffer_data[buff_idx.index()]));

            // Add position and normal coordinates.
            let iter = reader
                .read_positions()
                .unwrap()
                .zip(reader.read_normals().unwrap());
            for (position, normal) in iter {
                render_mesh.vertices.push(GpuVertex {
                    position,
                    color: render_data::DEFAULT_COLOR,
                    normal,
                    ..Default::default()
                });
            }

            // Add indices.
            render_mesh.indices = reader.read_indices().unwrap().into_u32().collect();

            // Assume first set of coords is for base color texture.
            if let Some(iter) = reader.read_tex_coords(0) {
                render_mesh
                    .vertices
                    .iter_mut()
                    .zip(iter.into_f32())
                    .for_each(|(vertex, tex_coords)| vertex.tex_coords = tex_coords);
            }

            let pbr_metallic = primitive.material().pbr_metallic_roughness();

            if let Some(info) = pbr_metallic.base_color_texture() {
                let image_source = info.texture().source().source();
                match image_source {
                    Source::Uri { uri, .. } => {
                        Self::indent(depth + 1);
                        println!("Mesh has texture: {uri}");
                    }
                    Source::View { .. } => {
                        Self::indent(depth + 1);
                        println!("Mesh has buffer view texture.");
                    }
                }
            }
        }
        println!();
    }
}
