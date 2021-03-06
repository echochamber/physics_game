pub type Color = [f32; 4];

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const ORANGE: Color =  [1.0, 0.5, 0.0, 1.0];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameColors {
    Red,
    Green,
    Blue,
    Yellow,
    Orange
}

// (x, y) coordinates
impl Into<Color> for GameColors {
    fn into(self) -> Color {
        match self {
            GameColors::Red => RED,
            GameColors::Green => GREEN,
            GameColors::Blue => BLUE,
            GameColors::Yellow => YELLOW,
            GameColors::Orange => ORANGE
        }
    }
}
