
/*********************************** MULTIMEDIA ***********************************/

use sdl2::{
    image,
    video::Window
};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use crate::UTILS::MISC_MATH::DegreesToRadians;

pub struct SDLContexts {
    pub sdlContext: sdl2::Sdl,
    pub sdlVideoSubsystem: sdl2::VideoSubsystem,
    pub sdlImageContext: image::Sdl2ImageContext,
}

impl SDLContexts {
    pub fn New() -> Self {
        let _sdlContext = sdl2::init().unwrap();
            _sdlContext.mouse().set_relative_mouse_mode(true);
        let _sdlVideoSubsystem = _sdlContext.video().unwrap();
        let _sdlImageContext = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

        Self {
            sdlContext: _sdlContext,
            sdlVideoSubsystem: _sdlVideoSubsystem,
            sdlImageContext: _sdlImageContext,
        }
    }

    pub fn CreateWindow(&self, title: &str, width: u32, height: u32) -> Window {
        let _sdlWindow = self.sdlVideoSubsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        return _sdlWindow;
    }
}

pub struct WindowParams {
    pub windowWidth: usize,
    pub windowHeight: usize,
}

pub struct RenderParams {
    pub fov: f64,
    pub castingRayAngles: Vec<(f64, f64)>
}

impl RenderParams {
    pub fn New(fov: f64, windowWidth: usize) -> Self {
        // Calculate casting ray angles
        let mut castingRayAngles: Vec<(f64, f64)> = vec![(0.0, 0.0); windowWidth];
        let projectionPlaneWidth: f64 = 2.0 * DegreesToRadians(fov / 2.0).tan();
        let segmentLength: f64 = projectionPlaneWidth / windowWidth as f64;
        for x in 0..windowWidth-1 {
            let currAngle = (-(x as f64 * segmentLength - (projectionPlaneWidth / 2.0))).atan();
            castingRayAngles[x] = (currAngle, currAngle.cos());
        }

        RenderParams {
            fov,
            castingRayAngles
        }
    }
}

pub struct Assets<'a> {
    pub wallTextures: Vec<Texture<'a>>
}

impl<'a> Assets<'a> {
    pub fn LoadWallTextures(sdlTextureCreator: &'a TextureCreator<WindowContext>) -> Self {
        let mut wallTextures: Vec<Texture> = Vec::new();
        let textureSheet = Surface::load_bmp("wall_textures.bmp").unwrap();

        for textureID in 1..110 {
            wallTextures.push(Self::ExtractTextureFromSurface(sdlTextureCreator, &textureSheet, textureID, 64));
        }

        Self {
            wallTextures
        }
    }

    fn ExtractTextureFromSurface(sdlTextureCreator: &'a TextureCreator<WindowContext>, textureSheet: &Surface, textureID: i32, texturePitch: i32) -> Texture<'a> {
        let textureSheetPitch = 6;
        let textureX = ((textureID - 1) % textureSheetPitch ) * texturePitch;
        let textureY = ((textureID - 1) / textureSheetPitch ) * texturePitch;

        let mut extractedTextureSurface = Surface::new(texturePitch as u32, texturePitch as u32, PixelFormatEnum::ARGB8888).unwrap();
        let _ = textureSheet.blit(Rect::new(textureX, textureY, texturePitch as u32, texturePitch as u32), &mut extractedTextureSurface, Rect::new(0, 0, texturePitch as u32, texturePitch as u32));

        return sdlTextureCreator.create_texture_from_surface(&extractedTextureSurface).unwrap();
    }
}