#[macro_use]
extern crate ecs;

extern crate piston_window;
extern crate piston;
extern crate rand;
extern crate sdl2_window;
extern crate nphysics;
extern crate nalgebra;
extern crate ncollide;
extern crate num;

use std::env::current_exe;
use piston_window::*;
use std::f64::consts::PI;
use sdl2_window::Sdl2Window;

mod game;
mod display;

fn main() {
    // Tinker with these for some fun
    let settings = game::GameSettings::default();

    // Create window
    let window_size = settings.size;
    let opengl = OpenGL::V3_2;
    let (width, height) = (window_size as u32, window_size as u32);
    let window: PistonWindow<(), Sdl2Window> =  WindowSettings::new("Physics game", (width, height + 20))
        .samples(4)
        .vsync(true)
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    // Create game 
    let resource_path = current_exe().unwrap().parent().unwrap().to_owned().join("resources/");

    let mut game = game::Game::new(settings);
    game.initialize_game();


    for e in window {
        match e.event {
            Some(Event::Input(Input::Press(Button::Keyboard(key)))) => {
                game.key_press(key);
            }

            Some(Event::Input(Input::Release(Button::Keyboard(key)))) => {
                game.key_release(key);
            },
            Some(Event::Input(Input::Move(Motion::MouseCursor(x, y)))) => {
                println!("Window pos: ({}, {})", x, y);
                let coor = game.camera.window_pos_to_coord(&::nalgebra::Vec2{x: x, y: y});
                println!("Coord: ({}, {})", coor.x, coor.y);
            }

            Some(Event::Render(_)) => {
                e.draw_2d(|c, g| {
                    game.render(c, g)
                });
                //game.pause();
            }

            Some(Event::Update(args)) => {
                game.update(args.dt);
            }

            _ => {}
        }
    }
}