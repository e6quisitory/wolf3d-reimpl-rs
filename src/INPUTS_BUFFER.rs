
/*********************************** INPUTS_BUFFER ***********************************/

#[derive(Default)]
pub enum lookCommand_t {
    RIGHT,
    LEFT,

    #[default]
    NONE
}

#[derive(Default)]
pub enum moveCommand_t {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    NORTH_EAST,
    NORTH_WEST,

    #[default]
    NONE
}

#[derive(Default)]
pub enum doorCommand_t {
    OPEN,

    #[default]
    NONE
}

#[derive(Default)]
pub struct InputsBuffer {
    pub lookCommand: lookCommand_t,
    pub moveCommand: moveCommand_t,
    pub doorCommand: doorCommand_t,
    pub quit: bool
}