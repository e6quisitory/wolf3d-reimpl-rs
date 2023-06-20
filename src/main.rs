mod utils;

use std::ops::Mul;
use std::time::Duration;
use ndarray::Array2;
use utils::mapCSV::*;
use sdl2::*;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::{LoadTexture, Sdl2ImageContext};
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::{Window, WindowContext};
use std::error::Error;
use crate::utils::conventions::PI;
use crate::utils::dda::{RayCursor, wallType_t};
use crate::utils::misc_math::DegreesToRadians;
use crate::utils::ray::Ray;
use crate::utils::vec2d::{Dot, Point2};

pub struct SdlContext {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    _image_context: sdl2::image::Sdl2ImageContext,
}

impl SdlContext {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let context = sdl2::init()?;
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

fn main() {
    let context = SdlContext::new().unwrap();
    let mut game = Game::new(context).unwrap();

    let mut canvas = game.window.into_canvas().build().unwrap();
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
    const windowWidth: usize = 1280;
    const windowHeight: usize = 720;
    let fov: f64 = DegreesToRadians(80.0);

    // Player info
    let mut playerPos = Point2::New(4.127, 5.033);
    let mut playerViewDir = Point2::New(-0.019038625821465295, 0.7068504302374231).UnitVector();

    //Pre-calculate angles
    let mut castingRayAngles: [(f64, f64); windowWidth] = [(0.0, 0.0); windowWidth];
    let projectionPlaneWidth: f64 = 2.0 * DegreesToRadians(fov / 2.0).tan();
    let segmentLength: f64 = projectionPlaneWidth / windowWidth as f64;
    for x in 0..windowWidth-1 {
        let currAngle = (-(x as f64 * segmentLength - (projectionPlaneWidth / 2.0))).atan();
        castingRayAngles[x] = (currAngle, currAngle.cos());
    }

    let mut event_pump = game.context.context.event_pump().unwrap();

    'running: loop {
        let playerEast = playerViewDir.Rotate(PI/2.0);
        let playerWest = playerViewDir.Rotate(-PI/2.0);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'running; },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => { playerViewDir = playerViewDir.Rotate((PI/20.0)*0.15); },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => { playerViewDir = playerViewDir.Rotate((-PI/20.0)*0.15); },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { playerPos = playerPos + playerViewDir*0.3*0.15; },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { playerPos = playerPos - playerViewDir*0.3*0.15; },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { playerPos = playerPos + playerEast*0.3*0.15; },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { playerPos = playerPos + playerWest*0.3*0.15; },


                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();

        for x in 0..windowWidth-1 {
            let mut currRay = Ray::New(playerPos, playerViewDir.Rotate(castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, playerPos);
            while (rayCursor.hitTile.x() >= 0 && rayCursor.hitTile.x() < mapWidth as i32) && (rayCursor.hitTile.y() >= 0 && rayCursor.hitTile.y() < mapHeight as i32) {
                rayCursor.GoToNextHit();
                if *(array.get((rayCursor.hitusTile.x() as usize, rayCursor.hitTile.y() as usize)).unwrap()) == 1 {
                    let dist = rayCursor.GetDistToHitPoint();
                    let renderHeight = (1000.0/(dist*castingRayAngles[x].1)) as usize;
                    if (rayCursor.GetWallType() == wallType_t::VERTICAL) {
                        canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                    } else {
                        canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
                    }

                    //canvas.fill_rect(Rect::new(x as i32, 0, 1, 10));
                    let mut y = ((windowHeight as f64 / 2.0) - (renderHeight as f64 / 2.0)) as i32;
                    canvas.fill_rect(Rect::new(x as i32, y, 1, renderHeight as u32)).unwrap();
                    break;
                }
            }
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        println!("{:?}", playerPos);
        println!("{:?}", playerViewDir);
    }
}


