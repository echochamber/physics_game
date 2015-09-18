use nalgebra;
use nphysics::object::RigidBodyHandle;
use display::colors::GameColors;
use piston_window::*;


pub struct Ball {
    body: RigidBodyHandle,
    radius: f64,
    color: GameColors,
    gfx: Ellipse
}

impl Ball {
    pub fn new(radius: f64, color: GameColors, body: RigidBodyHandle) -> Ball {
        Ball {
            body: body,
            radius: radius,
            gfx: Ellipse::new(color.into()),
            color: color
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        let body = self.body.borrow();
        let pos = nalgebra::translation(body.position());
        //let rot = nalgebra::rotation(body.position());

        self.gfx.draw(
            [
                pos.x - self.radius,
                pos.y - self.radius,
                self.radius * 2.0,
                self.radius * 2.0
            ],
            &c.draw_state,
            c.transform,
            g
        );
    }
}