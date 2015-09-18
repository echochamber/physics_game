use piston_window::*;
use piston;
use nalgebra::Vec2;

pub struct Camera {
    x_offset: i32,
    y_offset: i32,
    curr_zoom: f64,
    view_dims: piston::window::Size
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            x_offset: 0,
            y_offset: 0,
            curr_zoom: 1.0,
            view_dims: piston::window::Size{width: 800, height: 800 }
        }
    }

    pub fn update_view_dims(&mut self, view_dims: piston::window::Size) {
        self.view_dims = view_dims
    }

    pub fn handle_event(&mut self, event: &Event) {
        
    }

    pub fn map_pixel_to_coords(&mut self, pixel_pos: Vec2<i64>) -> Vec2<f64> {
        let mapped_coords = Vec2::new(
            (pixel_pos.x as f64 * self.curr_zoom - (self.view_dims.width as f64 / 2.0) + self.x_offset as f64),
            (pixel_pos.y as f64 * self.curr_zoom - (self.view_dims.height as f64 / 2.0) + self.y_offset as f64)
        );
        mapped_coords
    }
}
