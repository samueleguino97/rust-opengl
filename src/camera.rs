
use cgmath::{Vector3,vec3,Matrix4,point3,perspective,Deg};
use cgmath::prelude::*;
use crate::shader::Shader;
use glfw::{Window,Action};
pub struct Camera{
  pub position:Vector3<f32>,
  direction:Vector3<f32>,
  up:Vector3<f32>,
  fov:f32,
  near_plane:f32,
  far_plane:f32,
  pub camera_matrix:Matrix4<f32>,

  width:i32, height:i32,speed:f32,sensitivity:f32,

  first_mouse:bool,last_x:f32,last_y:f32,yaw:f32,pitch:f32
}
impl Camera {
  pub fn create(width:i32, height:i32,position:Vector3<f32>,fov:f32,near_plane:f32,far_plane:f32)->Self{
    Self {width,height,position,up:vec3(0.,1.0,0.),direction:vec3(0.,0.,-1.),fov,near_plane,far_plane,speed:0.05,sensitivity:0.05,camera_matrix:Matrix4::identity(),
    
      first_mouse:true,last_x:0.,last_y:0.,yaw:-90.,pitch:0.
    }
  }
  pub fn update_uniforms(&mut self,shader:&Shader){
  let camera_point = point3(self.position.x,self.position.y,self.position.z);
    let view:Matrix4<f32> = Matrix4::look_at_rh(camera_point, camera_point + self.direction,self.up);
    let proj:Matrix4<f32> =  perspective(Deg(self.fov),(self.width/self.height) as f32,self.near_plane,self.far_plane);

    shader.set_mat4f("camMatrix", proj* view);
    self.camera_matrix = proj * view
  }
  pub fn handle_input(&mut self,window:&mut Window){
    match window.get_key(glfw::Key::W) {
      Action::Press => {
        self.position += self.speed * self.direction;
      }
      _ => {}
    }
    match window.get_key(glfw::Key::S) {
        Action::Press => {
          self.position -= self.speed * self.direction;
        }
        _ => {}
    }
    match window.get_key(glfw::Key::A) {
        Action::Press => {
          self.position -=
                InnerSpace::normalize(Vector3::cross(self.direction, self.up)) * self.speed;
        }
        _ => {}
    }
    match window.get_key(glfw::Key::D) {
        Action::Press => {
          self.position +=
                InnerSpace::normalize(Vector3::cross(self.direction, self.up)) *  self.speed;
        }
        _ => {}
    }
    match window.get_key(glfw::Key::Space) {
      Action::Press => {
        self.position += self.speed * self.up;
      }
      _ => {}
    }
    match window.get_key(glfw::Key::LeftControl) {
      Action::Press => {
        self.position += self.speed * -self.up;
      }
      _ => {}
    }
    match window.get_key(glfw::Key::LeftShift) {
      Action::Press => {
        self.speed = 0.01;
      }
      Action::Release => {

        self.speed = 0.005;
      }
      _ => {}
    }
    match window.get_key(glfw::Key::Escape) {
      Action::Press => {
        window.set_cursor_mode(glfw::CursorMode::Normal);
      }
      _ => {}
    }

    //Mouse movement
    let (x,y)  = window.get_cursor_pos();

    if self.first_mouse{
      self.last_x = x as f32;
      self.last_y = y as f32;
      self.first_mouse = false;
    }

    let mut xoffset = x as f32 - self.last_x;
    let mut yoffset = y as f32 - self.last_y;

    self.last_x = x as f32;
    self.last_y = y as f32;

    xoffset *= self.sensitivity;
    yoffset *= self.sensitivity;

    self.yaw = (self.yaw + xoffset) % 360.;

    self.pitch -= yoffset;

    if self.pitch > 89.0 {
      self.pitch = 89.0;
    }
    if self.pitch < -89.0 {
      self.pitch = -89.0;
    }

    let direction = vec3(
      Angle::cos(Deg(self.yaw)) * Angle::cos(Deg(self.pitch)),
      Angle::sin(Deg(self.pitch)),
      Angle::sin(Deg(self.yaw)) * Angle::cos(Deg(self.pitch)),
    );
    self.direction = direction.normalize();

  }
}