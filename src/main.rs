mod UTILS;
mod MULTIMEDIA;
mod GAME_ENGINE;
mod INPUTS_BUFFER;
mod INPUTS_PARSER;
mod PLAYER;
mod MAP;

use UTILS::{
    DDA::{RayCursor, wallType_t},
    MISC_MATH::DegreesToRadians,
    RAY::Ray,
};
use sdl2::{
    rect::Rect,
    pixels::Color,
    keyboard::Keycode,
    event::Event::*,
};
use crate::GAME_ENGINE::GameEngine;
use crate::INPUTS_PARSER::ParseInputs;

fn main() {
    let mut gameEngine = GameEngine::New(1280, 720);

    // Window params
    let fov: f64 = 90.0;

    //Pre-calculate angles
    let mut castingRayAngles: Vec<(f64, f64)> = vec![(0.0, 0.0); gameEngine.windowParams.windowWidth];
    let projectionPlaneWidth: f64 = 2.0 * DegreesToRadians(fov / 2.0).tan();
    let segmentLength: f64 = projectionPlaneWidth / gameEngine.windowParams.windowWidth as f64;
    for x in 0..gameEngine.windowParams.windowWidth -1 {
        let currAngle = (-(x as f64 * segmentLength - (projectionPlaneWidth / 2.0))).atan();
        castingRayAngles[x] = (currAngle, currAngle.cos());
    }

    let mut event_pump = gameEngine.sdlContexts.sdlContext.event_pump().unwrap();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Quit {..} | KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running; },
                _ => {}
            }
        }

        ParseInputs(event_pump.keyboard_state(), &mut gameEngine.inputsBuffer);

        gameEngine.player.Update(&gameEngine.inputsBuffer);


        /************ Renderer ************/

        gameEngine.sdlCanvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        gameEngine.sdlCanvas.clear();

        for x in 0..gameEngine.windowParams.windowWidth -1 {
            let currRay = Ray::New(gameEngine.player.position, gameEngine.player.viewDir.Rotate(castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, gameEngine.player.position);
            while gameEngine.map.WithinMap(rayCursor.hitTile) {
                rayCursor.GoToNextHit();
                if gameEngine.map.GetTile(rayCursor.hitTile) == 1 {
                    let dist = rayCursor.GetDistToHitPoint();
                    let renderHeight = (400.0/(dist*castingRayAngles[x].1)) as usize;
                    if rayCursor.GetWallType() == wallType_t::VERTICAL {
                        gameEngine.sdlCanvas.set_draw_color(Color::RGBA(199, 199, 199, 255));
                    } else {
                        gameEngine.sdlCanvas.set_draw_color(Color::RGBA(81, 81, 81, 255));
                    }

                    //canvas.fill_rect(Rect::new(x as i32, 0, 1, 10));
                    let y = ((gameEngine.windowParams.windowHeight as f64 / 2.0) - (renderHeight as f64 / 2.0)) as i32;
                    gameEngine.sdlCanvas.fill_rect(Rect::new(x as i32, y, 1, renderHeight as u32)).unwrap();
                    break;
                }
            }
        }

        gameEngine.sdlCanvas.present();
    }
}


