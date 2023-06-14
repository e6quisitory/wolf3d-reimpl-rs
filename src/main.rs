mod utils;
use std::ops::Mul;
use std::time::Duration;
use ndarray::Array2;
use utils::mapCSV::*;
use sdl2::*;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::{LoadTexture, Sdl2ImageContext};
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::{Window, WindowContext};
use std::error::Error;

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
        let window = context.create_window("Game title", 800, 600)?;

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

    // Load the texture.
    let texture = texture_creator.load_texture("SS.png").unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));

    // Define the source rectangle (part of the texture to render).
    let source_rect = Rect::new(10, 10, 100, 100);

    // Define the destination rectangle (where to render it in the window).
    let dest_rect = Rect::new(50, 50, 200, 200);

    // Render a part of the texture to a part of the window.
    canvas.copy(&texture, Some(source_rect), Some(dest_rect)).expect("Render failed");

    canvas.present();

    let mut event_pump = game.context.context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // let array = match parseCSV("map.csv") {
    //     Ok(array) => array,
    //     Err(err) => {
    //         println!("Error: {}", err);
    //         return;
    //     },
    // };
    //
    // println!("{:?}", &array);
    // println!("{}, {}", array.nrows(), array.ncols());
}


