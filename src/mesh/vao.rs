
use glad_gl::gl;
use crate::mesh::vbo::VBO;

pub struct VAO {
  id:gl::GLuint
}
impl VAO {
  pub fn create()->Self{
    let mut id = 0;
    unsafe {
      gl::GenVertexArrays(1, &mut id);
    
    }
    VAO{id}
    // gl::BindVertexArray(VAO);
  }
  pub fn link_attrib(&self,vbo:&VBO,layout:gl::GLuint,num_components:gl::GLint,attrib_type:gl::GLenum,stride:usize,offset: usize){
    vbo.bind();
    unsafe {
    gl::VertexAttribPointer(layout, num_components, attrib_type, gl::GL_FALSE,stride as gl::GLsizei,  offset as *const gl::GLvoid);
    gl::EnableVertexAttribArray(layout);
    }
    vbo.unbind()
  }
  pub fn bind(&self){
    unsafe { 
      gl::BindVertexArray(self.id);
    }
  }
  pub fn unbind(&self){
    unsafe { 
      gl::BindVertexArray(0);
    }
  }
  pub fn delete(&self){
    unsafe {
      gl::DeleteVertexArrays(1, &self.id)};
  }
}