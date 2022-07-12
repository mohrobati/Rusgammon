pub mod checker;
pub mod dice;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::rectangle::square;
use std::path::Path;
use checker::{Checker, CheckerType};
use dice::{Dice};
use crate::drawer::Drawer;

pub struct Board {
    pub slots: Vec<Vec<Checker>>,
    pub bins: Vec<Vec<Checker>>,
    pub houses: Vec<Vec<Checker>>,
    pub dice: Dice,
    pub drawer: Drawer
}

impl Board {

    pub fn new() -> Self {
        let INIT_BOARD: [Vec<Checker>; 24] = [
            Vec::from([Checker {checker_type: CheckerType::First}; 2]),
            vec![],
            vec![],
            vec![],
            vec![],
            Vec::from([Checker {checker_type: CheckerType::Second}; 5]),
            vec![],
            Vec::from([Checker {checker_type: CheckerType::Second}; 3]),
            vec![],
            vec![],
            vec![],
            Vec::from([Checker {checker_type: CheckerType::First}; 5]),
            Vec::from([Checker {checker_type: CheckerType::Second}; 5]),
            vec![],
            vec![],
            vec![],
            Vec::from([Checker {checker_type: CheckerType::First}; 3]),
            vec![],
            Vec::from([Checker {checker_type: CheckerType::First}; 5]),
            vec![],
            vec![],
            vec![],
            vec![],
            Vec::from([Checker {checker_type: CheckerType::Second}; 2]),
        ];
        let mut slots: Vec<Vec<Checker>> = vec![];
        for i in 0..INIT_BOARD.len() {
            slots.push(vec![]);
            for j in 0..INIT_BOARD[i].len() {
                slots[i].push(INIT_BOARD[i][j])
            }
        }
        return Self { 
            slots: slots,
            bins: vec![],
            houses: vec![],
            dice: Dice::new(),
            drawer: Drawer::new()
        }
    }

    pub fn display(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;    
        gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
            self.drawer.board_image.draw(&self.drawer.board_texture, &DrawState {scissor: None, blend: None, stencil: None}, c.transform, gl);
            self.dice.display(&self.drawer, gl, args, c);
            for i in 0..self.slots.len() {
                for j in 0..self.slots[i].len() {
                    let coords = Drawer::get_pos(i as i32, j as i32);
                    let image = self.drawer.checker_image.rect(rectangle::rectangle_by_corners(coords.0, coords.1, coords.0 - 50.0, coords.1 + 50.0)); 
                    match self.slots[i][j].checker_type {
                        CheckerType::First => image.draw(&self.drawer.white_checker_texture, &DrawState {scissor: None, blend: Some(draw_state::Blend::Alpha), stencil: None}, c.transform, gl),
                        CheckerType::Second => image.draw(&self.drawer.red_checker_texture, &DrawState {scissor: None, blend: Some(draw_state::Blend::Alpha), stencil: None}, c.transform, gl)
                    }
                }
            }
        });
    }

}