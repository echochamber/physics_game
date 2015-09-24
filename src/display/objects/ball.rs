use nalgebra;
use nphysics::object::RigidBodyHandle;
use display::colors::GameColors;
use piston_window::*;
use display::camera::Camera;

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

    pub fn render(&self, camera: &Camera, c: Context, g: &mut G2d) {
        let body = self.body.borrow();

        let pos = nalgebra::translation(body.position());
        //let rot = nalgebra::rotation(body.position());

        let draw_pos = camera.coord_to_window_pos(&pos);

        let scaled_radius = camera.scale_by_zoom(self.radius);

        self.gfx.draw(
            [
                draw_pos.x - scaled_radius,
                draw_pos.y - scaled_radius,
                scaled_radius * 2.0,
                scaled_radius * 2.0
            ],
            &c.draw_state,
            c.transform,
            g
        );
    }
}