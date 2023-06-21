
/*********************************** INPUTS_BUFFER ***********************************/

use sdl2::keyboard::{KeyboardState, Scancode};
use crate::INPUTS_BUFFER::InputsBuffer;
use super::INPUTS_BUFFER::{lookCommand_t, moveCommand_t};

pub fn ParseInputs(ks: KeyboardState, inputsBuffer: &mut InputsBuffer) {
    let LEFT = ks.is_scancode_pressed(Scancode::Left);
    let RIGHT = ks.is_scancode_pressed(Scancode::Right);
    let W = ks.is_scancode_pressed(Scancode::W);
    let S = ks.is_scancode_pressed(Scancode::S);
    let A = ks.is_scancode_pressed(Scancode::A);
    let D = ks.is_scancode_pressed(Scancode::D);

    if W && A { inputsBuffer.moveCommand = moveCommand_t::NORTH_WEST; }
    else if W && D { inputsBuffer.moveCommand = moveCommand_t::NORTH_EAST; }
    else if W { inputsBuffer.moveCommand = moveCommand_t::NORTH; }
    else if S { inputsBuffer.moveCommand = moveCommand_t::SOUTH; }
    else if A { inputsBuffer.moveCommand = moveCommand_t::WEST; }
    else if D { inputsBuffer.moveCommand = moveCommand_t::EAST; }
    else { inputsBuffer.moveCommand = moveCommand_t::NONE; }

    if LEFT { inputsBuffer.lookCommand = lookCommand_t::LEFT; }
    else if RIGHT { inputsBuffer.lookCommand = lookCommand_t::RIGHT; }
    else { inputsBuffer.lookCommand = lookCommand_t::NONE; }
}