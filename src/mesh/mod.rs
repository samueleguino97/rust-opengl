pub mod ebo;
pub mod vao;
pub mod vbo;

pub use self::ebo::EBO;
pub use self::vao::VAO;
pub use self::vbo::VBO;

use crate::camera::Camera;
use crate::shader::Shader;
use crate::texture::Texture;
use cgmath::{Matrix4, Quaternion, Vector2, Vector3};
use glad_gl::gl;

#[derive(Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub color: Vector3<f32>,
    pub tex_uv: Vector2<f32>,
}
impl Vertex {
    pub fn mem_size() -> usize {
        return std::mem::size_of::<Vertex>();
    }
}
pub struct Mesh {
    _vertices: Vec<Vertex>,
    _indices: Vec<gl::GLuint>,
    _textures: Vec<Texture>,
    vao: VAO,
    vbo: VBO,
    ebo: EBO,
}
impl Mesh {
    pub fn create(
        vertices: &Vec<Vertex>,
        indices: &Vec<gl::GLuint>,
        textures: &Vec<Texture>,
    ) -> Mesh {
        let new_mesh = Mesh {
            _vertices: vertices.clone(),
            _indices: indices.clone(),
            _textures: textures.clone(),
            vao: VAO::create(),
            vbo: VBO::create(&vertices),
            ebo: EBO::create(&indices),
        };
        new_mesh.vao.bind();
        new_mesh.ebo.bind();

        //Attribs
        //Position
        new_mesh
            .vao
            .link_attrib(&new_mesh.vbo, 0, 3, gl::GL_FLOAT, Vertex::mem_size(), 0);
        new_mesh.vao.link_attrib(
            &new_mesh.vbo,
            1,
            3,
            gl::GL_FLOAT,
            Vertex::mem_size(),
            3 * std::mem::size_of::<f32>(),
        );
        new_mesh.vao.link_attrib(
            &new_mesh.vbo,
            2,
            3,
            gl::GL_FLOAT,
            Vertex::mem_size(),
            6 * std::mem::size_of::<f32>(),
        );
        new_mesh.vao.link_attrib(
            &new_mesh.vbo,
            3,
            2,
            gl::GL_FLOAT,
            Vertex::mem_size(),
            9 * std::mem::size_of::<f32>(),
        );

        new_mesh.vao.unbind();
        new_mesh.vbo.unbind();
        new_mesh.ebo.unbind();
        new_mesh
    }

    pub fn draw(
        &self,
        shader: &Shader,
        camera: &Camera,
        matrix: Matrix4<f32>,
        translation: Vector3<f32>,
        rotation: Quaternion<f32>,
        scale: Vector3<f32>,
    ) {
        shader.use_shader();
        self.vao.bind();

        let mut num_diffuse = 0;
        let mut num_spec = 0;

        for (i, texture) in self._textures.iter().enumerate() {
            let mut num = String::new();
            let tex_type = &texture.texture_type;
            if tex_type == "diffuse" {
                num_diffuse += 1;
                num = num_diffuse.to_string();
            } else if tex_type == "specular" {
                num_spec += 1;
                num = num_spec.to_string();
            }

            let uniform_name = String::from(tex_type) + &num;

            shader.set_1i(&uniform_name, i as i32);
            texture.bind();
        }
        shader.set_vec3f("camPos", camera.position);
        shader.set_mat4f("camMatrix", camera.camera_matrix);

        let trans = Matrix4::from_translation(translation);
        let rot = Matrix4::from(rotation);
        let sca = Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);

        shader.set_mat4f("translation", trans);
        shader.set_mat4f("rotation", rot);
        shader.set_mat4f("scale", sca);
        shader.set_mat4f("model", matrix);

        unsafe {
            gl::DrawElements(
                gl::GL_TRIANGLES,
                self._indices.len() as gl::GLint,
                gl::GL_UNSIGNED_INT,
                0 as *const std::os::raw::c_void,
            );
        }
    }

    pub fn cleanup(&self) {
        self.vao.delete();
        self.vbo.delete();
        self.ebo.delete();
    }
}
