
/*********************************** CONVENTIONS ***********************************/

pub const PI: f64                 = std::f64::consts::PI;
pub const TEXTURE_PITCH: u32      = 64;
pub const TRANSPARENCY_COLOR: u32 = 0xFF980088;

#[derive(Default, Copy, Clone)]
pub enum xDir_t {
    EAST = 1,
    WEST = -1,

    #[default]
    NONE = 0
}

#[derive(Default, Copy, Clone)]
pub enum yDir_t {
    NORTH = 1,
    SOUTH = -1,

    #[default]
    NONE = 0
}

#[derive(Default, Copy, Clone)]
pub enum swivelDir_t {
    COUNTER_CLOCKWISE = 1,
    CLOCKWISE         = -1,

    #[default]
    NONE = 0
}