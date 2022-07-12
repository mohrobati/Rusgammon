use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::rectangle::square;
use std::path::Path;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CheckerType {
    First,
    Second,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Checker {
    pub checker_type: CheckerType, 
}