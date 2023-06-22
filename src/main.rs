mod UTILS;
mod MULTIMEDIA;
mod INPUTS_BUFFER;
mod PLAYER;
mod MAP;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::INPUTS_BUFFER::InputsBuffer;
use crate::MAP::Map;
use crate::MULTIMEDIA::{Assets, RenderParams, SDLContexts, WindowParams};
use crate::PLAYER::Player;
use crate::UTILS::DDA::{RayCursor, wallType_t};
use crate::UTILS::RAY::Ray;

fn main() {

    let windowWidth = 960;
    let windowHeight = 540;

    let sdlContexts = SDLContexts::New();
    let mut sdlCanvas = sdlContexts
        .CreateWindow("Wolfenstein 3D Clone - Rust", windowWidth as u32, windowHeight as u32)
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let mut sdlEventPump = sdlContexts.sdlContext.event_pump().unwrap();
    let windowParams = WindowParams { windowWidth, windowHeight };
    let mut inputsBuffer = InputsBuffer::default();
    let mut player = Player::New();
    let map = Map::LoadFromCSV("map.csv");
    let renderParams = RenderParams::New(90.0, windowWidth);
    let sdlTextureCreator = sdlCanvas.texture_creator();
    let assets = Assets::LoadWallTextures(&sdlTextureCreator);

    loop {
        inputsBuffer.Update(&mut sdlEventPump);
        player.Update(&inputsBuffer, &map);

        if inputsBuffer.quit {
            break;
        }

        sdlCanvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        sdlCanvas.clear();

        sdlCanvas.set_draw_color(Color::RGBA(50, 50, 50, 255));
        sdlCanvas.fill_rect(Rect::new(0, 0, windowWidth as u32, (windowHeight/2) as u32)).unwrap();

        sdlCanvas.set_draw_color(Color::RGBA(96, 96, 96, 255));
        sdlCanvas.fill_rect(Rect::new(0, (windowHeight / 2) as i32, windowWidth as u32, (windowHeight/2) as u32)).unwrap();

        for x in 0..windowParams.windowWidth -1 {
            let currRay = Ray::New(player.position, player.viewDir.Rotate(renderParams.castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, player.position);
            while map.WithinMap(rayCursor.hitTile) {
                rayCursor.GoToNextHit();
                let textureID = map.GetTile(rayCursor.hitTile);
                if textureID != 0 {
                    let dist = rayCursor.GetDistToHitPoint();

                    let propr_const = 1.15 * (windowWidth as f64) / ((16.0 / 9.0) * (renderParams.fov / 72.0));

                    let renderHeight = propr_const / (dist * renderParams.castingRayAngles[x].1);
                    if rayCursor.GetWallType() == wallType_t::VERTICAL {
                        sdlCanvas.set_draw_color(Color::RGBA(199, 199, 199, 255));
                    } else {
                        sdlCanvas.set_draw_color(Color::RGBA(81, 81, 81, 255));
                    }

                    //canvas.fill_rect(Rect::new(x as i32, 0, 1, 10));
                    let y = ((windowParams.windowHeight as f64 / 2.0) - (renderHeight / 2.0)) as i32;
                    //sdlCanvas.fill_rect(Rect::new(x as i32, y, 1, renderHeight as u32)).unwrap();
                    let _ = sdlCanvas.copy(&assets.wallTextures[textureID as usize], Rect::new((rayCursor.GetWidthPercent() as f64 * 64.0) as i32, 0, 1, 64),Rect::new(x as i32, y, 1, renderHeight as u32));
                    break;
                }
            }
        }

        sdlCanvas.present();
        
    }
}


