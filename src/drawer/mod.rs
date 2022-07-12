
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::*;
use std::path::Path;
use crate::board::Board;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;
const PANEL_POS: [(f64, f64); 4] = [(718.0, 20.0), (383.0, 20.0), (383.0-250.0, 530.0), (718.0-250.0, 530.0)];


pub struct Drawer {
    pub board_image: Image,
    pub checker_image: Image,
    pub dice_image: Image,
    pub board_texture: Texture,
    pub red_checker_texture: Texture,
    pub white_checker_texture: Texture,
    pub dice_texture: [Texture; 6],
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            board_image: Image::new().rect(rectangle::rectangle_by_corners(0.0, 0.0, WIDTH, HEIGHT)),
            checker_image: Image::new(),
            dice_image: Image::new().rect(rectangle::rectangle_by_corners(0.0, 0.0, 10.0, 10.0)),
            board_texture: Texture::from_path(Path::new("res/board.jpg"), &TextureSettings::new()).unwrap(),
            red_checker_texture: Texture::from_path(Path::new("res/red_checker.png"), &TextureSettings::new()).unwrap(),
            white_checker_texture: Texture::from_path(Path::new("res/white_checker.png"), &TextureSettings::new()).unwrap(),
            dice_texture: [
                Texture::from_path(Path::new("res/die_1.png"), &TextureSettings::new()).unwrap(),
                Texture::from_path(Path::new("res/die_2.png"), &TextureSettings::new()).unwrap(),
                Texture::from_path(Path::new("res/die_3.png"), &TextureSettings::new()).unwrap(),
                Texture::from_path(Path::new("res/die_4.png"), &TextureSettings::new()).unwrap(),
                Texture::from_path(Path::new("res/die_5.png"), &TextureSettings::new()).unwrap(),
                Texture::from_path(Path::new("res/die_6.png"), &TextureSettings::new()).unwrap(),
            ]
        }
    }

    pub fn get_pos(slot_id: i32, vertical_pos: i32) -> (f64, f64) {
        let panel_id: f64 = (((slot_id / 6) as f64).floor());
        let panel_offset: f64 = (slot_id % 6) as f64;
        if panel_id < 2.0 {
            return (PANEL_POS[panel_id as usize].0 - panel_offset*50.0, 
                PANEL_POS[panel_id as usize].1 + vertical_pos as f64 *50.0) 
        } else {
            return (PANEL_POS[panel_id as usize].0 + panel_offset*50.0, 
                PANEL_POS[panel_id as usize].1 - vertical_pos as f64 *50.0) 
        }
    }

    pub fn is_collided(&self, axis: &[f64; 2]) -> i32 {
        if axis[0] < PANEL_POS[0].0 && axis[0] > PANEL_POS[0].0 - 300.0 &&
            axis[1] > PANEL_POS[0].1 && axis[1] < PANEL_POS[0].1 + 250.0 {
                return ((axis[0] - PANEL_POS[0].0).abs() / 300.0 * 6.0 + 1.0) as i32;
        }
        if axis[0] < PANEL_POS[1].0 && axis[0] > PANEL_POS[1].0 - 300.0 &&
            axis[1] > PANEL_POS[1].1 && axis[1] < PANEL_POS[1].1 + 250.0 {
                return ((axis[0] - PANEL_POS[1].0).abs() / 300.0 * 6.0 + 7.0) as i32;
        }
        if axis[0] > PANEL_POS[2].0 - 50.0 && axis[0] < PANEL_POS[2].0 + 250.0 &&
            axis[1] < PANEL_POS[2].1 + 50.0 && axis[1] > PANEL_POS[2].1 - 200.0 {
                return ((axis[0] - (PANEL_POS[2].0 - 50.0)).abs() / 300.0 * 6.0 + 13.0) as i32;
        }
        if axis[0] > PANEL_POS[3].0 - 50.0 && axis[0] < PANEL_POS[3].0 + 250.0 &&
            axis[1] < PANEL_POS[3].1 + 50.0 && axis[1] > PANEL_POS[3].1 - 200.0 {
                return ((axis[0] - (PANEL_POS[3].0 - 50.0)).abs() / 300.0 * 6.0 + 19.0) as i32;
        }
        return 0;
    }

}