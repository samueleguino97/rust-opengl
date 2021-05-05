use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Vector4};
use glad_gl::gl;
use std::ffi::CString;
use std::fs;
use std::ptr::null;
use std::ptr::null_mut;

macro_rules! c_str {
    ($a:expr) => {
        std::ffi::CString::new($a).expect("CString::new failed")
    };
}

pub struct Shader {
    vertex_shader_path: String,
    fragment_shader_path: String,
    pub id: u32,
}
impl Shader {
    pub fn create(vertex_path: &str, fragment_path: &str) -> Self {
        let mut shader = Shader {
            vertex_shader_path: vertex_path.to_string(),
            fragment_shader_path: fragment_path.to_string(),
            id: 0,
        };

        let vertex_shader = shader.load_shader(&shader.vertex_shader_path, gl::GL_VERTEX_SHADER);
        let frag_shader = shader.load_shader(&shader.fragment_shader_path, gl::GL_FRAGMENT_SHADER);

        unsafe {
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, frag_shader);
            gl::LinkProgram(shader_program);

            shader.id = shader_program;

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(frag_shader);
        }
        shader
    }

    fn load_shader(&self, path: &str, shader_type: gl::GLenum) -> gl::GLuint {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let shader_contents = fs::read_to_string(path).expect("Error reading file");
        let c_str = CString::new(shader_contents).expect("CString::new failed");
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), null());
            gl::CompileShader(shader);

            let mut is_compiled: gl::GLint = 0;
            gl::GetShaderiv(shader, gl::GL_COMPILE_STATUS, &mut is_compiled);
            //Error logs
            if is_compiled == gl::GL_FALSE as i32 {
                let mut info_log: i8 = 1;
                gl::GetShaderInfoLog(shader, 512, null_mut(), &mut info_log);

                // Provide the infolog in whatever manor you deem best.
                // Exit with failure.
                print!("Error, Shader couldnt be compiled!: {}", path);
                print!("{:?}", info_log);
                gl::DeleteShader(self.id); // Don't leak the shader.
            }
        }
        shader
    }

    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unuse_shader(&self) {
        unsafe { gl::UseProgram(0) }
    }
    pub fn delete(&self) {
        unsafe { gl::DeleteShader(self.id) }
    }

    pub fn set_1i(&self, name: &str, value: gl::GLint) {
        unsafe {
            let location =
                gl::GetUniformLocation(self.id, c_str!(name).as_ptr() as *const gl::GLchar);
            self.use_shader();
            gl::Uniform1i(location, value);
        }
    }
    pub fn set_mat4f(&self, name: &str, value: Matrix4<f32>) {
        unsafe {
            let location =
                gl::GetUniformLocation(self.id, c_str!(name).as_ptr() as *const gl::GLchar);
            self.use_shader();
            gl::UniformMatrix4fv(location, 1, gl::GL_FALSE, value.as_ptr());
        }
    }
    pub fn set_vec4f(&self, name: &str, value: Vector4<f32>) {
        unsafe {
            let location =
                gl::GetUniformLocation(self.id, c_str!(name).as_ptr() as *const gl::GLchar);
            self.use_shader();
            gl::Uniform4fv(location, 1, value.as_ptr());
        }
    }
    pub fn set_vec3f(&self, name: &str, value: Vector3<f32>) {
        unsafe {
            let location =
                gl::GetUniformLocation(self.id, c_str!(name).as_ptr() as *const gl::GLchar);
            self.use_shader();
            gl::Uniform3fv(location, 1, value.as_ptr());
        }
    }
}
