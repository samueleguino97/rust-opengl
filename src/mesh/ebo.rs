use glad_gl::gl;

pub struct EBO {
    id: gl::GLuint,
}
impl EBO {
    pub fn create(indices: &Vec<gl::GLuint>) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::GL_ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(
                gl::GL_ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<gl::GLuint>()) as gl::GLsizeiptr,
                indices.as_ptr() as *const gl::GLvoid,
                gl::GL_STATIC_DRAW,
            );
        }

        EBO { id }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::GL_ELEMENT_ARRAY_BUFFER, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::GL_ELEMENT_ARRAY_BUFFER, 0);
        }
    }
    pub fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
