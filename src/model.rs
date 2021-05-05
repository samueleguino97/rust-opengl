use crate::camera::Camera;
use crate::mesh::{Mesh, Vertex};
use crate::shader::Shader;
use crate::texture::Texture;
use cgmath::prelude::*;
use cgmath::{vec2, vec3, Matrix4, Quaternion, Vector2, Vector3};
use gltf::buffer::Data;
use gltf::image::Source;
use gltf::Mesh as GLTFMesh;
use gltf::{Document, Node};
pub struct Model {
    meshes: Vec<Mesh>,
    textures: Vec<Texture>,
    buffers: Vec<Data>,
    trans_meshes: Vec<Vector3<f32>>,
    rot_meshes: Vec<Quaternion<f32>>,
    scale_meshes: Vec<Vector3<f32>>,
    matrix_meshes: Vec<Matrix4<f32>>,
}
impl Model {
    pub fn from_gltf(file_path: &str) -> Self {
        let mut new_model = Model {
            meshes: Vec::new(),
            buffers: Vec::new(),
            rot_meshes: Vec::new(),
            scale_meshes: Vec::new(),
            trans_meshes: Vec::new(),
            textures: Vec::new(),
            matrix_meshes: Vec::new(),
        };
        new_model.load_data();
        new_model
    }
    fn load_data(&mut self) {
        let (gltf, buffers, _) = gltf::import("res/models/ToyCar.glb").expect("Couldnt read model");

        self.buffers = buffers;
        self.load_textures(&gltf);

        for node in gltf.nodes() {
            self.load_node(node, Matrix4::identity());
        }
    }
    fn load_mesh(&mut self, mesh: GLTFMesh) {
        let mut positions: Vec<Vector3<f32>> = Vec::new();
        let mut normals: Vec<Vector3<f32>> = Vec::new();
        let mut tex_uvs: Vec<Vector2<f32>> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&self.buffers[buffer.index()]));
            if let Some(iter) = reader.read_positions() {
                for vertex_position in iter {
                    positions.push(Vector3::from(vertex_position));
                }
            }
            if let Some(iter) = reader.read_normals() {
                for vertex_normal in iter {
                    normals.push(Vector3::from(vertex_normal));
                }
            }
            if let Some(iter) = reader.read_tex_coords(0) {
                for vertex_uvs in iter.into_f32() {
                    tex_uvs.push(Vector2::from(vertex_uvs));
                }
            }
            indices = if let Some(indices) = reader.read_indices() {
                Some(indices.into_u32().map(|i| i as u32).collect()).unwrap()
            } else {
                Vec::new()
            };
        }

        let mut vertices: Vec<Vertex> = Vec::new();
        for (i, pos) in positions.iter().enumerate() {
            vertices.push(Vertex {
                position: *pos,
                normal: *normals.get(i).unwrap(),
                tex_uv: *tex_uvs.get(i).unwrap(),
                color: vec3(1.0, 1.0, 1.0),
            })
        }
        self.meshes
            .push(Mesh::create(&vertices, &indices, &self.textures));
    }

    fn load_node(&mut self, node: Node, matrix: Matrix4<f32>) {
        let transform = node.transform().decomposed();
        let mat_4 = node.transform().matrix();

        let node_matrix = Matrix4::from(mat_4);

        let trans = Matrix4::from_translation(Vector3::from(transform.0));
        let rot = Matrix4::from(Quaternion::new(
            *transform.1.get(3).unwrap(),
            *transform.1.get(0).unwrap(),
            *transform.1.get(1).unwrap(),
            *transform.1.get(2).unwrap(),
        ));

        let scale = Vector3::from(transform.2);
        let sca = Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);

        let mat_next: Matrix4<f32> = matrix * node_matrix * trans * rot * sca;
        if let Some(node_mesh) = node.mesh() {
            self.trans_meshes.push(Vector3::from(transform.0));
            self.rot_meshes.push(Quaternion::new(
                *transform.1.get(3).unwrap(),
                *transform.1.get(0).unwrap(),
                *transform.1.get(1).unwrap(),
                *transform.1.get(2).unwrap(),
            ));
            self.scale_meshes.push(Vector3::from(transform.2));
            self.matrix_meshes.push(mat_next);
            self.load_mesh(node_mesh);
        }

        for child in node.children() {
            self.load_node(child, mat_next);
        }
    }

    fn load_textures(&mut self, gltf: &Document) {
        let mut textures: Vec<Texture> = Vec::new();
        let mut tex_unit = 0;
        gltf.images().for_each(|img| match img.source() {
            Source::Uri { uri, .. } => {
                let tex_type = {
                    if uri.contains("baseColor") {
                        "diffuse";
                        tex_unit += 1;
                    }
                    if uri.contains("metallicRoughness") {
                        "specular";

                        tex_unit += 1;
                    }
                    ""
                };
                let texture = Texture::from_file(uri, tex_type, tex_unit);
                textures.push(texture);
            }
            _ => {}
        });
    }

    pub fn draw(&mut self, shader: &Shader, camera: &Camera) {
        for (i, mesh) in self.meshes.iter().enumerate() {
            mesh.draw(
                shader,
                camera,
                *self.matrix_meshes.get(i).unwrap(),
                *self.trans_meshes.get(i).unwrap(),
                *self.rot_meshes.get(i).unwrap(),
                *self.scale_meshes.get(i).unwrap(),
            );
        }
    }
}
