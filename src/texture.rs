use std::ffi::CString;

use glad_gl::gl;
#[derive(Clone)]
pub struct Texture {
    width: i32,
    height: i32,
    id: u32,
    pub texture_type: String,
    pub slot: gl::GLuint,
}
impl Texture {
    pub fn from_file(file_path: &str, texture_type: &str, slot: gl::GLuint) -> Self {
        let mut new_texture = Texture {
            width: 0,
            height: 0,
            id: 0,
            texture_type: texture_type.to_string(),
            slot,
        };

        let mut bpp: i32 = 0;
        //Load file
        let path_as_c_str = CString::new(file_path).expect("Couldnt convert to cstring");
        unsafe {
            stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
            let img = stb_image::stb_image::bindgen::stbi_load(
                path_as_c_str.as_ptr(),
                &mut new_texture.width,
                &mut new_texture.height,
                &mut bpp,
                4,
            );

            gl::GenTextures(1, &mut new_texture.id);
            gl::ActiveTexture(gl::GL_TEXTURE0 + new_texture.slot);
            gl::BindTexture(gl::GL_TEXTURE_2D, new_texture.id);

            //Texture Options
            gl::TexParameteri(
                gl::GL_TEXTURE_2D,
                gl::GL_TEXTURE_MIN_FILTER,
                gl::GL_NEAREST as i32,
            );
            gl::TexParameteri(
                gl::GL_TEXTURE_2D,
                gl::GL_TEXTURE_MAG_FILTER,
                gl::GL_NEAREST as i32,
            );
            gl::TexParameteri(
                gl::GL_TEXTURE_2D,
                gl::GL_TEXTURE_WRAP_S,
                gl::GL_REPEAT as i32,
            );
            gl::TexParameteri(
                gl::GL_TEXTURE_2D,
                gl::GL_TEXTURE_WRAP_T,
                gl::GL_REPEAT as i32,
            );
            //Load Texture
            gl::TexImage2D(
                gl::GL_TEXTURE_2D,
                0,
                gl::GL_RGBA as i32,
                new_texture.width,
                new_texture.width,
                0,
                gl::GL_RGBA,
                gl::GL_UNSIGNED_BYTE,
                img as *const gl::types::GLvoid,
            );

            gl::GenerateMipmap(gl::GL_TEXTURE_2D);
            //Clean up

            gl::ActiveTexture(0);
            gl::BindTexture(gl::GL_TEXTURE_2D, 0);
            stb_image::stb_image::bindgen::stbi_image_free(img as *mut gl::types::GLvoid);
        }
        new_texture
    }
    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::GL_TEXTURE0 + self.slot);
            gl::BindTexture(gl::GL_TEXTURE_2D, self.id);
        }
    }
    // pub fn unbind(&self){
    //   unsafe {
    //     gl::BindTexture(self.texture_type,0);

    //   }
    // }
}
