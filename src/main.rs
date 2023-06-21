mod utils;

use std::collections::HashMap;
use utils::mapCSV::*;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::video::Window;
use std::error::Error;
use sdl2::event;
use sdl2::event::Event::*;
use crate::utils::conventions::PI;
use crate::utils::dda::{RayCursor, wallType_t};
use crate::utils::misc_math::DegreesToRadians;
use crate::utils::ray::Ray;
use crate::utils::vec2d::Point2;

pub struct SdlContext {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    _image_context: sdl2::image::Sdl2ImageContext,
}

impl SdlContext {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let context = sdl2::init()?;
        context.mouse().set_relative_mouse_mode(true);
        let video_subsystem = context.video()?;
        let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)?;

        Ok(Self {
            context,
            video_subsystem,
            _image_context,
        })
    }

    pub fn create_window(&self, title: &str, width: u32, height: u32) -> Result<Window, Box<dyn Error>> {
        let window = self.video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()?;

        Ok(window)
    }
}

pub struct Game {
    context: SdlContext,
    window: Window,
}

impl Game {
    pub fn new(context: SdlContext) -> Result<Self, Box<dyn Error>> {
        let window = context.create_window("Wolfenstein 3D Clone - Rust", 1280, 720)?;

        Ok(Self {
            context,
            window,
        })
    }
}

#[derive(Default)]
enum lookCommand_t {
    RIGHT,
    LEFT,

    #[default]
    NONE
}

#[derive(Default)]
enum moveCommand_t {
    NORTH,
    SOUTH,
    EAST,
    WEST,

    #[default]
    NONE
}

#[derive(Default)]
enum doorCommand_t {
    OPEN,

    #[default]
    NONE
}

#[derive(Default)]
struct InputsBuffer {
    lookCommand: lookCommand_t,
    moveCommand: moveCommand_t,
    doorCommand: doorCommand_t
}

fn main() {
    let context = SdlContext::new().unwrap();
    let game = Game::new(context).unwrap();

    let mut canvas = game.window.into_canvas().accelerated().present_vsync().build().unwrap();
    //let texture_creator = canvas.texture_creator();

    // Load the texture
    //let texture = texture_creator.load_texture("SS.png").unwrap();

    // Inputs
    let mut inputsBuffer: InputsBuffer = InputsBuffer::default();

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

    // Player info
    let mut playerPos = Point2::New(4.127, 5.033);
    let mut playerViewDir = Point2::New(-0.019038625821465295, 0.7068504302374231).UnitVector();

    //Pre-calculate angles
    let mut castingRayAngles: [(f64, f64); WINDOW_WIDTH] = [(0.0, 0.0); WINDOW_WIDTH];
    let projectionPlaneWidth: f64 = 2.0 * DegreesToRadians(fov / 2.0).tan();
    let segmentLength: f64 = projectionPlaneWidth / WINDOW_WIDTH as f64;
    for x in 0..WINDOW_WIDTH -1 {
        let currAngle = (-(x as f64 * segmentLength - (projectionPlaneWidth / 2.0))).atan();
        castingRayAngles[x] = (currAngle, currAngle.cos());
    }

    let mut event_pump = game.context.context.event_pump().unwrap();

    'running: loop {

        let mut currXrel = 0;

        for event in event_pump.poll_iter() {
            match event {
                Quit {..} | KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running; },
                MouseMotion {xrel, .. } => {
                    currXrel = xrel;

                }
                _ => {}
            }
        }

        let playerEast = playerViewDir.Rotate(-PI/2.0);
        let playerWest = playerViewDir.Rotate(PI/2.0);

        let swivelSpeed = 0.25;
        let moveSpeed = 0.1;

        let ks = event_pump.keyboard_state();
        let LEFT = ks.is_scancode_pressed(Scancode::Left);
        let RIGHT = ks.is_scancode_pressed(Scancode::Right);
        let W = ks.is_scancode_pressed(Scancode::W);
        let S = ks.is_scancode_pressed(Scancode::S);
        let A = ks.is_scancode_pressed(Scancode::A);
        let D = ks.is_scancode_pressed(Scancode::D);

        if W && A {
            playerPos = playerPos + playerViewDir*moveSpeed*0.7071067 + playerWest*moveSpeed*0.7071067;
        } else if W && D {
            playerPos = playerPos + playerViewDir*moveSpeed*0.7071067 + playerEast*moveSpeed*0.7071067;
        } else if W {
            playerPos = playerPos + playerViewDir*moveSpeed;
        } else if S {
            playerPos = playerPos - playerViewDir*moveSpeed;
        } else if A {
            playerPos = playerPos + playerWest*moveSpeed;
        } else if D {
            playerPos = playerPos + playerEast*moveSpeed;
        }

        if LEFT {
            playerViewDir = playerViewDir.Rotate((PI/20.0)*swivelSpeed);
        } else if RIGHT {
            playerViewDir = playerViewDir.Rotate((-PI/20.0)*swivelSpeed);
        }

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();

        for x in 0..WINDOW_WIDTH -1 {
            let currRay = Ray::New(playerPos, playerViewDir.Rotate(castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, playerPos);
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


