mod UTILS;

use std::error::Error;
use sdl2::{
    rect::Rect,
    pixels::Color,
    keyboard::{Keycode, Scancode},
    video::{Window, WindowContext},
    image::{self, LoadTexture},
    event::Event::*,
    render::{Texture, TextureCreator},
    surface::Surface,
};
use UTILS::{
    CSV::*,
    CONVENTIONS::PI,
    DDA::{RayCursor, wallType_t},
    MISC_MATH::DegreesToRadians,
    RAY::Ray,
    VEC2D::Point2,
};

pub struct SdlContext {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    _image_context: image::Sdl2ImageContext,
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
    NORTH_EAST,
    NORTH_WEST,

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
    doorCommand: doorCommand_t,
    quit: bool
}

fn main() {
    let context = SdlContext::new().unwrap();
    let game = Game::new(context).unwrap();

    let mut canvas = game.window.into_canvas().accelerated().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Load the texture
    let texture = texture_creator.load_texture("SS.png").unwrap();

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

        for event in event_pump.poll_iter() {
            match event {
                Quit {..} | KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running; },
                _ => {}
            }
        }

        /************ Inputs Parser ************/

        let mut currXrel = 0;

        let playerEast = playerViewDir.Rotate(-PI/2.0);
        let playerWest = playerViewDir.Rotate(PI/2.0);

        let swivelIncr = 0.25*PI/20.0;
        let moveIncr = 0.1;

        let ks = event_pump.keyboard_state();
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

        /************ Player Manager ************/

        match inputsBuffer.moveCommand {
            moveCommand_t::NORTH => {
                playerPos += playerViewDir* moveIncr;
            }
            moveCommand_t::SOUTH => {
                playerPos -= playerViewDir* moveIncr;
            }
            moveCommand_t::EAST => {
                playerPos += playerEast* moveIncr;
            }
            moveCommand_t::WEST => {
                playerPos += playerWest* moveIncr;
            }
            moveCommand_t::NORTH_EAST => {
                playerPos += playerViewDir* moveIncr *0.7071067 + playerEast* moveIncr *0.7071067;
            }
            moveCommand_t::NORTH_WEST => {
                playerPos += playerViewDir* moveIncr *0.7071067 + playerWest* moveIncr *0.7071067;
            }
            moveCommand_t::NONE => {}
        }

        match inputsBuffer.lookCommand {
            lookCommand_t::RIGHT => {
                playerViewDir = playerViewDir.Rotate(-swivelIncr);
            }
            lookCommand_t::LEFT => {
                playerViewDir = playerViewDir.Rotate(swivelIncr);
            }
            lookCommand_t::NONE => {}
        }

        /************ Renderer ************/

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


