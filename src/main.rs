mod utils;

use std::time::Duration;
use ndarray::Array2;
use utils::mapCSV::*;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

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

    let mut event_pump = sdl_context.event_pump().unwrap();

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