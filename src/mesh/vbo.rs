use crate::mesh::Vertex;
use glad_gl::gl;
pub struct VBO {
    id: gl::GLuint,
}
impl VBO {
    pub fn create(vertices: &Vec<Vertex>) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::GL_ARRAY_BUFFER, id);
            gl::BufferData(
                gl::GL_ARRAY_BUFFER,
                (vertices.len() * Vertex::mem_size()) as gl::GLsizeiptr,
                vertices.as_ptr() as *const gl::GLvoid,
                gl::GL_STATIC_DRAW,
            );
        }

        VBO { id }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::GL_ARRAY_BUFFER, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::GL_ARRAY_BUFFER, 0);
        }
    }
    pub fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
