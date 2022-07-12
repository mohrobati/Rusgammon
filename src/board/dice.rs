use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use graphics::{Context, DrawState, rectangle, draw_state};
use piston::window::WindowSettings;
use graphics::rectangle::square;
use std::path::Path;
use crate::drawer::Drawer;
use rand::prelude::*;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 550.0;
const DIE_SIZE: f64 = 40.0;
const SPLIT: f64 = 5.0;

pub struct Dice {
    dice: [i32; 2],
    first: bool
}

impl Dice {

    pub fn new() -> Self {
        let mut dice = Self {
            dice: [1,1],
            first: true
        };
        dice.throw();
        return dice;
    }

    pub fn throw(&mut self) {
        let mut rng = rand::thread_rng();
        let mut y: f64 = rng.gen(); // generates a float between 0 and 1
        self.dice[0] = (y * 6.0).floor() as i32;
        y = rng.gen();
        self.dice[1] = (y as f64 * 6.0).floor() as i32;
    }

    pub fn get_active(&mut self) -> (i32, bool) {
        match self.first {
            true => {
                return (self.dice[0] + 1, false);
            },
            false => {
                return (self.dice[1] + 1, true);
            }
        }
    }    

    pub fn spend_die(&mut self) {
        self.first = !self.first
    }

    pub fn swap(&mut self) {
        let mut tmp: i32 = 1;
        tmp = self.dice[0];
        self.dice[0] = self.dice[1];
        self.dice[1] = tmp;
    }

    pub fn display(&self, drawer: &Drawer, gl: &mut GlGraphics, args: &RenderArgs, c: Context) {
        let die_1 = drawer.dice_image.rect(rectangle::rectangle_by_corners(
            WIDTH/2.0 + 0.0 - DIE_SIZE/2.0,
            HEIGHT/2.0 + 0.0 - DIE_SIZE/2.0,
            WIDTH/2.0 + DIE_SIZE - DIE_SIZE/2.0,
            HEIGHT/2.0 + DIE_SIZE - DIE_SIZE/2.0)
        ); 
        let die_2 = drawer.dice_image.rect(rectangle::rectangle_by_corners(
            WIDTH/2.0 + 0.0 - DIE_SIZE/2.0,
            HEIGHT/2.0 + DIE_SIZE + SPLIT - DIE_SIZE/2.0,
            WIDTH/2.0 + DIE_SIZE - DIE_SIZE/2.0,
            HEIGHT/2.0 + 2.0*DIE_SIZE + SPLIT - DIE_SIZE/2.0)
        );         
        die_1.draw(&drawer.dice_texture[self.dice[0] as usize], &DrawState {scissor: None, blend: Some(draw_state::Blend::Alpha), stencil: None}, c.transform, gl);
        die_2.draw(&drawer.dice_texture[self.dice[1] as usize], &DrawState {scissor: None, blend: Some(draw_state::Blend::Alpha), stencil: None}, c.transform, gl);
    }

    pub fn is_collided(&self, axis: &[f64; 2]) -> bool {
        (axis[0] > WIDTH/2.0 + 0.0 - DIE_SIZE/2.0) && (axis[1] < HEIGHT/2.0 + 2.0*DIE_SIZE + SPLIT - DIE_SIZE/2.0) &&
        (axis[0] < WIDTH/2.0 + DIE_SIZE - DIE_SIZE/2.0) && (axis[1] > HEIGHT/2.0 + 0.0 - DIE_SIZE/2.0) 
    }
}