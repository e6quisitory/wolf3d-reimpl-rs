mod UTILS;
mod MULTIMEDIA;
mod GAME_ENGINE;
mod INPUTS_BUFFER;
mod INPUTS_PARSER;
mod PLAYER;

use UTILS::{
    CSV::*,
    DDA::{RayCursor, wallType_t},
    MISC_MATH::DegreesToRadians,
    RAY::Ray,
};
use sdl2::{
    rect::Rect,
    pixels::Color,
    keyboard::Keycode,
    image::LoadTexture,
    event::Event::*,
};
use crate::GAME_ENGINE::GameEngine;
use crate::INPUTS_PARSER::ParseInputs;
use crate::MULTIMEDIA::SDLContexts;

fn main() {
    let sdlContexts = SDLContexts::New();
    let mut gameEngine = GameEngine::New(sdlContexts);

    let mut canvas = gameEngine.sdlWindow.into_canvas().accelerated().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Load the texture
    let texture = texture_creator.load_texture("SS.png").unwrap();

    // Load map
    let array = match parseCSV("map.csv") {
        Ok(array) => array,
        Err(err) => {
            println!("Error: {}", err);
            return;
        },
    };
    let mapWidth = array.nrows();
    let mapHeight = array.ncols();
    println!("{}", array.get((1,1)).unwrap());

    // Window params
    const WINDOW_WIDTH: usize = 1280;
    const WINDOW_HEIGHT: usize = 720;
    let fov: f64 = 90.0;

    //Pre-calculate angles
    let mut castingRayAngles: [(f64, f64); WINDOW_WIDTH] = [(0.0, 0.0); WINDOW_WIDTH];
    let projectionPlaneWidth: f64 = 2.0 * DegreesToRadians(fov / 2.0).tan();
    let segmentLength: f64 = projectionPlaneWidth / WINDOW_WIDTH as f64;
    for x in 0..WINDOW_WIDTH -1 {
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

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();

        for x in 0..WINDOW_WIDTH -1 {
            let currRay = Ray::New(gameEngine.player.position, gameEngine.player.viewDir.Rotate(castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, gameEngine.player.position);
            while (rayCursor.hitTile.x() >= 0 && rayCursor.hitTile.x() < mapWidth as i32) && (rayCursor.hitTile.y() >= 0 && rayCursor.hitTile.y() < mapHeight as i32) {
                rayCursor.GoToNextHit();
                if *(array.get((rayCursor.hitTile.x() as usize, rayCursor.hitTile.y() as usize)).unwrap()) == 1 {
                    let dist = rayCursor.GetDistToHitPoint();
                    let renderHeight = (400.0/(dist*castingRayAngles[x].1)) as usize;
                    if rayCursor.GetWallType() == wallType_t::VERTICAL {
                        canvas.set_draw_color(Color::RGBA(199, 199, 199, 255));
                    } else {
                        canvas.set_draw_color(Color::RGBA(81, 81, 81, 255));
                    }

                    //canvas.fill_rect(Rect::new(x as i32, 0, 1, 10));
                    let y = ((WINDOW_HEIGHT as f64 / 2.0) - (renderHeight as f64 / 2.0)) as i32;
                    canvas.fill_rect(Rect::new(x as i32, y, 1, renderHeight as u32)).unwrap();
                    break;
                }
            }
        }

        canvas.present();
    }
}


