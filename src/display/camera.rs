use piston_window::*;
use piston;
use nalgebra::Vec2;

pub type Coord = Vec2<f64>;

pub struct Camera {
    center: Coord,
    curr_zoom: f64,
    view_dims: piston::window::Size
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            center: Coord { x: 0.0, y: 0.0},
            curr_zoom: 1.0,
            view_dims: piston::window::Size{width: width, height: height}
        }
    }

    pub fn update_view_dims(&mut self, view_dims: piston::window::Size) {
        self.view_dims = view_dims
    }

    pub fn set_center(&mut self, x: f64, y: f64) {
        self.center.x = x;
        self.center.y = y;
    }

    pub fn offset_center(&mut self, x: f64, y: f64) {
        self.center.x += x;
        self.center.y += y;
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        self.curr_zoom = zoom;
    }

    pub fn window_pos_to_coord(&self, window_pos: &Vec2<f64>) -> Vec2<f64> {
        Vec2::new(
            self.center.x + (window_pos.x - self.view_dims.width as f64 / 2.0) / self.curr_zoom,
            self.center.y + (window_pos.y - self.view_dims.height as f64 / 2.0) / self.curr_zoom
        )
    }

    pub fn coord_to_window_pos(&self, coord: &Vec2<f64>) -> Vec2<f64> {
        Vec2::new(
            self.curr_zoom * (coord.x - self.center.x) + (self.view_dims.width as f64 / 2.0),
            self.curr_zoom * (coord.y - self.center.y) + (self.view_dims.height as f64 / 2.0)
        )
    }

    pub fn scale_by_zoom(&self, val: f64) -> f64 {
        val * self.curr_zoom
    }

    pub fn inverse_scale_by_zoom(&self, val: f64) -> f64 {
        val / self.curr_zoom
    }
}
