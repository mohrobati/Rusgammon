mod board;
mod drawer;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::rectangle::square;
use piston::input::*;
use std::path::Path;
use board::Board;


const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    board: Board,  // Rotation for the square.
}

pub struct State {
    slot_id: i32,
    turn: board::checker::CheckerType
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.board.display(&mut self.gl, args);
    }

    fn swap_turn(&self, state: &mut State) {
        if state.turn == board::checker::CheckerType::First {
            state.turn = board::checker::CheckerType::Second;
            return;
        } 
        state.turn = board::checker::CheckerType::First;
    }

    fn update(&mut self, args: &UpdateArgs, state: &mut State) {
        if state.slot_id > 0 && self.board.slots[(state.slot_id-1) as usize].len() != 0 &&
            self.board.slots[(state.slot_id-1) as usize][0].checker_type == state.turn {
                let (die, done) = self.board.dice.get_active();
                if (self.board.slots[(die + (state.slot_id-1)) as usize].len() == 0 || 
                    self.board.slots[(die + (state.slot_id-1)) as usize][0].checker_type == state.turn) {
                        let checker = self.board.slots[(state.slot_id-1) as usize].pop().unwrap();
                        self.board.slots[(die + (state.slot_id-1)) as usize].push(checker);
                        state.slot_id = 0;
                        self.board.dice.spend_die();
                        if done { 
                            self.swap_turn(state);
                            self.board.dice.throw();
                        }
                }
            }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut cursor = [0.0, 0.0];
    let mut state = State {
        slot_id: 0,
        turn: board::checker::CheckerType::First
    };
    let mut window: Window = WindowSettings::new("spinning-square", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        board: Board::new(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(Button::Mouse(button)) = e.press_args() {
            if app.board.dice.is_collided(&cursor) {
                app.board.dice.swap();
            }
            state.slot_id = app.board.drawer.is_collided(&cursor);
        }

        e.mouse_cursor(|pos| {
            cursor = pos;
        });

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &mut state);
        }
    }
}
