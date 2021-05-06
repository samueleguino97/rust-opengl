extern crate cgmath;
extern crate glad_gl;
extern crate glfw;
extern crate stb_image;
use glad_gl::gl;

use glfw::Context;

mod camera;
mod mesh;
mod model;
mod shader;
mod texture;

use camera::Camera;
use model::Model;
use shader::Shader;

use cgmath::{vec3, vec4};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, _events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "OpenGL Projects",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    gl::load(|param| return glfw.get_proc_address_raw(param));
    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);

        // gl::Enable(gl::GL_BLEND);
        // gl::BlendFunc(gl::GL_SRC_ALPHA, gl::GL_ONE_MINUS_SRC_ALPHA);

        gl::Enable(gl::GL_DEPTH_TEST);
    }

    let shader = Shader::create("res/shaders/default.vert", "res/shaders/default.frag");

    let light_color = vec4(1.0, 1.0, 1.0, 1.0);
    let light_pos = vec3(0.0, 1.5, 0.5);

    let mut camera = Camera::create(
        SCREEN_WIDTH as i32,
        SCREEN_HEIGHT as i32,
        vec3(0., 0.5, 3.),
        45.,
        0.01,
        100.,
    );

    let mut sword = Model::from_gltf("res/models/sword/scene.gltf");
    // let mut anime = Model::from_gltf("res/models/anime/scene.gltf");
    shader.use_shader();
    shader.set_vec4f("lightColor", light_color);
    shader.set_vec3f("lightPos", light_pos);

    window.set_cursor_mode(glfw::CursorMode::Disabled);
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 0.5);
            gl::Clear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);
        }

        camera.update_uniforms(&shader);
        camera.handle_input(&mut window);

        sword.draw(&shader, &camera);

        //Draw
        window.swap_buffers();
        glfw.poll_events();
    }

    shader.unuse_shader();
    shader.delete();
}
