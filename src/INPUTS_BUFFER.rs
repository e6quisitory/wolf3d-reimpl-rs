
/*********************************** INPUTS_BUFFER ***********************************/

use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use sdl2::{
    keyboard::Keycode,
    event::Event::*,
};

#[derive(Default, Debug)]
pub enum lookCommand_t {
    RIGHT,
    LEFT,

    #[default]
    NONE
}

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub enum doorCommand_t {
    OPEN,

    #[default]
    NONE
}

#[derive(Default, Debug)]
pub struct InputsBuffer {
    pub lookCommand: lookCommand_t,
    pub moveCommand: moveCommand_t,
    pub doorCommand: doorCommand_t,
    pub quit: bool
}

impl InputsBuffer {
    pub fn Update(&mut self, sdlEventPump: &mut EventPump) {
        for event in sdlEventPump.poll_iter() {
            match event {
                Quit {..} | KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.quit = true;
                    break;
                },
                _ => {}
            }
        }

        let ks = sdlEventPump.keyboard_state();

        let LEFT = ks.is_scancode_pressed(Scancode::Left);
        let RIGHT = ks.is_scancode_pressed(Scancode::Right);
        let W = ks.is_scancode_pressed(Scancode::W);
        let S = ks.is_scancode_pressed(Scancode::S);
        let A = ks.is_scancode_pressed(Scancode::A);
        let D = ks.is_scancode_pressed(Scancode::D);
        let SPACE = ks.is_scancode_pressed(Scancode::Space);

        if W && A { self.moveCommand = moveCommand_t::NORTH_WEST; }
        else if W && D { self.moveCommand = moveCommand_t::NORTH_EAST; }
        else if W { self.moveCommand = moveCommand_t::NORTH; }
        else if S { self.moveCommand = moveCommand_t::SOUTH; }
        else if A { self.moveCommand = moveCommand_t::WEST; }
        else if D { self.moveCommand = moveCommand_t::EAST; }
        else { self.moveCommand = moveCommand_t::NONE; }

        if LEFT { self.lookCommand = lookCommand_t::LEFT; }
        else if RIGHT { self.lookCommand = lookCommand_t::RIGHT; }
        else { self.lookCommand = lookCommand_t::NONE; }

        self.doorCommand = if SPACE {
            doorCommand_t::OPEN
        } else {
            doorCommand_t::NONE
        };
    }
}