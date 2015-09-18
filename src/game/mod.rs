use piston_window::*;
use nphysics::world::World;
use nphysics::object::RigidBody;
use ncollide;
use nalgebra::{Vec2, Iso2};
use nalgebra;
use num::One;
use display::objects::Ball;

use display;

pub mod ecs;

pub struct Game {
    settings: GameSettings,
    world: World,
    paused: bool,
    balls: Vec<Ball>
}

impl Game {
    pub fn new(settings: GameSettings) -> Game {
        let mut world = World::new();
        world.set_gravity(Vec2::new(0.0, settings.gravity));

        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(Vec2::new(-1.0, -1.0)), 0.3, 0.6);
        rb.append_translation(&Vec2::new(600.0, 600.0));
        world.add_body(rb);

        let mut rb = RigidBody::new_static(ncollide::shape::Plane::new(Vec2::new(1.0, -1.0)), 0.3, 0.6);
        rb.append_translation(&Vec2::new(600.0, 600.0));
        world.add_body(rb);

        let num     = 20usize;
        let rad     = 0.5;
        let shift   = 2.5 * rad;
        let centerx = shift * (num as f64) / 2.0;
        let centery = shift * (num as f64) / 2.0;

        let mut ball_vec = Vec::with_capacity(300);
        for i in 0usize .. num {
            for j in 0usize .. num {
                let x = i as f64 * 2.5 * rad - centerx;
                let y = j as f64 * 2.5 * rad - centery * 2.0 - 20.0;

                let mut rb = RigidBody::new_dynamic(ncollide::shape::Ball::new(1.0), 1.0, 0.3, 0.6);

                rb.append_translation(&Vec2::new(x, y));
                let handle = world.add_body(rb);
                ball_vec.push(display::objects::Ball::new(10.0, display::GameColors::Orange, handle))
            }
        }

        Game {
            settings: settings,
            world: world,
            paused: false,
            balls: ball_vec
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
            self.world.step(dt * self.settings.speed);
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        if self.paused {
            return;
        }

        clear([1.0; 4], g);

        let border = Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 2.0);
        border.draw(
            [0.0, 0.0, self.settings.size, self.settings.size],
            &c.draw_state,
            c.transform,
            g
        );

        for ball in &self.balls {
            ball.render(c, g);
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
            size: 800.0,
            gravity: 9.81,
            speed: 2.25
        }
    }
}