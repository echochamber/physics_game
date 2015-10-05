use piston_window::*;
use nphysics::world::World;
use nphysics::object::RigidBody;
use ncollide;
use nalgebra::{Vec2, Iso2};
use nalgebra;
use num::One;
use display::objects::Ball;
use display::camera::Camera;
use display::camera::Coord;

use display;

pub mod ecs;

pub struct Game {
    settings: GameSettings,
    world: World,
    paused: bool,
    balls: Vec<Ball>,
    pub camera: Camera,
    timer: f64,
    temp: f64
}

impl Game {
    pub fn new(settings: GameSettings) -> Game {
        let mut world = World::new();
        world.set_gravity(Vec2::new(0.0, settings.gravity));

        Game {
            settings: settings,
            world: world,
            paused: false,
            balls: Vec::with_capacity(300),
            camera: Camera::new(620, 620),
            timer: 0.0,
            temp: 0.0
        }
    }
}

impl Game {
    pub fn key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, key: Key) {
        //self.handle_key(key, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Space => {
                self.paused = !self.paused;
            }
            _ => ()
        }
    }

    pub fn update(&mut self, dt: f64) {
        if !self.paused {
            self.timer += dt;
            self.world.step(dt * self.settings.speed);

            if self.timer - self.temp > 1.0 {
                self.temp = self.timer;
                self.paused = true;
            }
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }

    pub fn render(&mut self, c: Context, g: &mut G2d) {
        if self.paused {
            return;
        }

        //let context = self.camera.update_context(&c);

        clear([1.0; 4], g);
        let border = Rectangle::new_border([0.0, 0.0, 0.0, 1.0], self.camera.scale_by_zoom(2.0));
        let border_corner = self.camera.coord_to_window_pos(&Coord {x: -200.0, y: -200.0});
        let border_size = self.camera.scale_by_zoom(self.settings.size - 200.0);
        //println!("{:?} {:?}", border_corner, border_size);
        border.draw(
            [border_corner.x, border_corner.y, border_size, border_size],
            &c.draw_state,
            c.transform,
            g
        );

        for ball in &self.balls {
            ball.render(&self.camera, c, g);
        }
    }
    pub fn initialize_game(&mut self) {
        self.camera.set_zoom(0.5);
        //self.camera.offset_center(100.0, 100.0);

        let plane_trans = self.camera.scale_by_zoom(300.0);

        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(Vec2::new(-1.0, -1.0)), 0.3, 0.6);
        //rb.append_translation(&Vec2::new(plane_trans, plane_trans));
        self.world.add_body(rb);

        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(Vec2::new(1.0, -1.0)), 0.3, 0.6);
        //rb.append_translation(&Vec2::new(plane_trans, plane_trans));
        self.world.add_body(rb);

        let num     = 20usize;
        let rad     = 0.5;
        let shift   = 2.5 * rad;
        let centerx = shift * (num as f64) / 2.0;
        let centery = shift * (num as f64) / 2.0;
        
        for i in 0usize .. 1usize {
            for j in 0usize .. 1usize {
                let x = i as f64 * 2.5 * rad - centerx;
                let y = j as f64 * 2.5 * rad - centery * 2.0 - 20.0;

                let mut rb = RigidBody::new_dynamic(ncollide::shape::Ball::new(1.0), 1.0, 0.3, 0.6);

                rb.append_translation(&Vec2::new(self.camera.scale_by_zoom(x) + 000.0, self.camera.scale_by_zoom(y)+ 000.0 ));
                rb.append_translation(&Vec2::new(-plane_trans, -plane_trans));
                let handle = self.world.add_body(rb);
                self.balls.push(display::objects::Ball::new(10.0, display::GameColors::Orange, handle))
            }
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameSettings {
    pub size: f64,
    pub gravity: f64,
    pub speed: f64
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            size: 600.0,
            gravity: 9.81,
            speed: 2.25
        }
    }
}