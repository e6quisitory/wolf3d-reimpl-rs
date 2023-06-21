
/*********************************** INPUTS_BUFFER ***********************************/

use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use super::INPUTS_BUFFER::{lookCommand_t, moveCommand_t};
use super::UTILS::CONVENTIONS::PI;
use super::INPUTS_BUFFER::InputsBuffer;

pub fn ParseInputs(sdlEventPump: &EventPump, inputsBuffer: &mut InputsBuffer) {
    let ks = sdlEventPump.keyboard_state();

    let LEFT = ks.is_scancode_pressed(Scancode::Left);
    let RIGHT = ks.is_scancode_pressed(Scancode::Right);
    let W = ks.is_scancode_pressed(Scancode::W);
    let S = ks.is_scancode_pressed(Scancode::S);
    let A = ks.is_scancode_pressed(Scancode::A);
    let D = ks.is_scancode_pressed(Scancode::D);

    if W && A {
        inputsBuffer.moveCommand = moveCommand_t::NORTH_WEST;
    } else if W && D {
        inputsBuffer.moveCommand = moveCommand_t::NORTH_EAST;
    } else if W {
        inputsBuffer.moveCommand = moveCommand_t::NORTH;
    } else if S {
        inputsBuffer.moveCommand = moveCommand_t::SOUTH;
    } else if A {
        inputsBuffer.moveCommand = moveCommand_t::WEST;
    } else if D {
        inputsBuffer.moveCommand = moveCommand_t::EAST;
    } else {
        inputsBuffer.moveCommand = moveCommand_t::NONE;
    }

    if LEFT {
        inputsBuffer.lookCommand = lookCommand_t::LEFT;
    } else if RIGHT {
        inputsBuffer.lookCommand = lookCommand_t::RIGHT;
    } else {
        inputsBuffer.lookCommand = lookCommand_t::NONE;
    }
}