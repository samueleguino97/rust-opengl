extern crate cgmath;
extern crate glad_gl;
extern crate glfw;
extern crate stb_image;
#[macro_use]
extern crate serde_json;
use glad_gl::gl;

use glfw::Context;

mod camera;
mod mesh;
mod model;
mod shader;
mod texture;

use camera::Camera;
use mesh::{Mesh, Vertex};
use shader::Shader;
use texture::Texture;

use cgmath::prelude::*;
use cgmath::{vec2, vec3, vec4, Matrix4};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

fn main() {
    //Vertices
    let verts: Vec<Vertex> = vec![
        // Bottom Face
        Vertex {
            position: vec3(-0.5, 0.0, 0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, -1., 0.0),
            tex_uv: vec2(0.0, 0.0),
        },
        Vertex {
            position: vec3(-0.5, 0.0, -0.5),
            color: vec3(0., 1., 0.0),
            normal: vec3(0.0, -1., 0.0),
            tex_uv: vec2(0.0, 5.0),
        },
        Vertex {
            position: vec3(0.5, 0.0, -0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, -1., 0.0),
            tex_uv: vec2(5.0, 5.0),
        },
        Vertex {
            position: vec3(0.5, 0.0, 0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, -1., 0.0),
            tex_uv: vec2(5.0, 0.0),
        },
        // Left Side
        Vertex {
            position: vec3(-0.5, 0.0, 0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(-0.8, 0.5, 0.0),
            tex_uv: vec2(0.0, 0.0),
        },
        Vertex {
            position: vec3(-0.5, 0.0, -0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(-0.8, 0.5, 0.0),
            tex_uv: vec2(5.0, 0.0),
        },
        Vertex {
            position: vec3(0.0, 0.8, 0.0),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(-0.8, 0.5, 0.0),
            tex_uv: vec2(2.5, 5.0),
        },
        // Back Face
        Vertex {
            position: vec3(-0.5, 0.0, -0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, -0.8),
            tex_uv: vec2(0.0, 0.0),
        },
        Vertex {
            position: vec3(0.5, 0.0, -0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, -0.8),
            tex_uv: vec2(5.0, 0.0),
        },
        Vertex {
            position: vec3(0.0, 0.8, 0.0),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, -0.8),
            tex_uv: vec2(2.5, 5.0),
        },
        //Right Face
        Vertex {
            position: vec3(0.5, 0.0, -0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.8, 0.5, 0.0),
            tex_uv: vec2(0.0, 0.0),
        },
        Vertex {
            position: vec3(0.5, 0.0, 0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.8, 0.5, 0.0),
            tex_uv: vec2(5.0, 0.0),
        },
        Vertex {
            position: vec3(0.0, 0.8, 0.0),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.8, 0.5, 0.0),
            tex_uv: vec2(2.5, 5.0),
        },
        //Front
        Vertex {
            position: vec3(0.5, 0.0, 0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(0.0, 0.0),
        },
        Vertex {
            position: vec3(-0.5, 0.0, 0.5),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(5.0, 0.0),
        },
        Vertex {
            position: vec3(0.0, 0.8, 0.0),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(2.5, 5.0),
        },
    ];
    let ind: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 7, 9, 8, 10, 12, 11, 13, 15, 14];

    let cube_verts: Vec<Vertex> = vec![
        Vertex {
            position: vec3(-0.1, -0.1, 0.1),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(2.5, 5.0),
        },
        Vertex {
            position: vec3(-0.1, -0.1, -0.1),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(2.5, 5.0),
        },
        Vertex {
            position: vec3(0.1, -0.1, -0.1),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(2.5, 5.0),
        },
        Vertex {
            position: vec3(0.1, -0.1, 0.1),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(2.5, 5.0),
        },
        Vertex {
            position: vec3(-0.1, 0.1, 0.1),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(2.5, 5.0),
        },
        Vertex {
            position: vec3(-0.1, 0.1, -0.1),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(2.5, 5.0),
        },
        Vertex {
            position: vec3(0.1, 0.1, -0.1),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(2.5, 5.0),
        },
        Vertex {
            position: vec3(0.1, 0.1, 0.1),
            color: vec3(1., 0.0, 0.0),
            normal: vec3(0.0, 0.5, 0.8),
            tex_uv: vec2(2.5, 5.0),
        },
    ];
    let cube_ind: Vec<u32> = vec![
        0, 1, 2, 0, 2, 3, 0, 4, 7, 0, 7, 3, 3, 7, 6, 3, 6, 2, 2, 6, 5, 2, 5, 1, 1, 5, 4, 1, 4, 0,
        4, 5, 6, 4, 6, 7,
    ];

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
    let light_shader = Shader::create("res/shaders/light.vert", "res/shaders/light.frag");

    let textures: Vec<Texture> = vec![
        Texture::from_file("res/textures/plank.jpg", "diffuse", 0),
        Texture::from_file("res/textures/plank.spec.jpg", "specular", 1),
    ];

    let mut pyramid = Mesh::create(&verts, &ind, &textures);

    let mut light_cube = Mesh::create(&cube_verts, &cube_ind, &textures);

    let light_color = vec4(1.0, 1.0, 1.0, 1.0);
    let light_pos = vec3(0.0, 1.5, 0.5);
    let mut light_model: Matrix4<f32> = Matrix4::identity();
    light_model = light_model * Matrix4::from_translation(light_pos);

    let pyramid_pos = vec3(0.0, 0.0, 0.0);
    let mut pyramid_model: Matrix4<f32> = Matrix4::identity();
    pyramid_model = pyramid_model * Matrix4::from_translation(pyramid_pos);

    let mut camera = Camera::create(
        SCREEN_WIDTH as i32,
        SCREEN_HEIGHT as i32,
        vec3(0., 0.5, 3.),
        45.,
        0.01,
        100.,
    );

    light_shader.use_shader();
    light_shader.set_mat4f("model", light_model);
    light_shader.set_vec4f("lightColor", light_color);

    shader.use_shader();
    shader.set_mat4f("model", pyramid_model);
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

        //Draw
        pyramid.draw(&shader, &camera);
        light_cube.draw(&light_shader, &camera);
        window.swap_buffers();
        glfw.poll_events();
    }
    pyramid.cleanup();
    light_cube.cleanup();

    shader.unuse_shader();
    shader.delete();
}
