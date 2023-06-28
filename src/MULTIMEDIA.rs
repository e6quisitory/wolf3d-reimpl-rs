
/*********************************** MULTIMEDIA ***********************************/

use sdl2::EventPump;
use sdl2::{
    image,
    video::Window
};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::{Surface};
use sdl2::video::WindowContext;
use crate::tiles::TexturePair;
use crate::utils::dda::RayCursor;
use crate::utils::misc_math::DegreesToRadians;

pub struct Multimedia {
    pub sdlContexts: SDLContexts,
    pub sdlEventPump: EventPump,
    pub sdlCanvas: WindowCanvas,
    pub sdlTextureCreator: TextureCreator<WindowContext>,
    pub displayParams: DisplayParams,
    pub windowParams: WindowParams,
    pub renderParams: RenderParams,
    pub assets: Assets,
}

impl Multimedia {
    pub fn New(windowWidth: usize, windowHeight: usize, fov: f64) -> Self {
        let sdlContexts = SDLContexts::New();
        let displayMode = sdlContexts.sdlVideoSubsystem.current_display_mode(0).unwrap();
        let sdlEventPump = sdlContexts.sdlContext.event_pump().unwrap();
        let sdlCanvas = sdlContexts
            .CreateWindow("Wolfenstein 3D Clone - Rust", windowWidth as u32, windowHeight as u32)
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();
        let sdlTextureCreator = sdlCanvas.texture_creator();
        let displayParams = DisplayParams {
            width: displayMode.w as usize,
            height: displayMode.h as usize,
            refreshRate: displayMode.refresh_rate as usize
        };
        let windowParams = WindowParams{width: windowWidth, height: windowHeight};
        let renderParams = RenderParams::New(fov, windowWidth);
        let assets = Assets::LoadWallTextures(&sdlTextureCreator);


        return Self {
            sdlContexts,
            sdlEventPump,
            sdlCanvas,
            sdlTextureCreator,
            displayParams,
            windowParams,
            renderParams,
            assets,
        }
    }
}

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

pub struct DisplayParams {
    pub width: usize,
    pub height: usize,
    pub refreshRate: usize
}

pub struct WindowParams {
    pub width: usize,
    pub height: usize,
}

pub struct RenderParams {
    pub fov: f64,
    pub castingRayAngles: Vec<(f64, f64)>,
    pub renderHeightProprConst: f64
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
        
        // Render height proportionality constant ; takes into account screen aspect ratio
        let renderHeightProprConst = 1.15 * (windowWidth as f64) / ((16.0 / 9.0) * (fov / 72.0));  

        RenderParams {
            fov,
            castingRayAngles,
            renderHeightProprConst
        }
    }
}

pub struct Assets {
    pub wallTextures: Vec<Texture>,
    pub gateSidewallTexturePair: TexturePair
}

impl Assets {
    pub fn GetWallTexture(&self, textureID: i32) -> &Texture {
        return &self.wallTextures[(textureID-1) as usize];
    }

    pub fn LoadWallTextures(sdlTextureCreator: &TextureCreator<WindowContext>) -> Self {
        let mut wallTextures: Vec<Texture> = Vec::new();
        let textureSheet = Surface::load_bmp("wall_textures.bmp").unwrap();

        for textureID in 1..=110 {
            wallTextures.push(Self::ExtractTextureFromSurface(sdlTextureCreator, &textureSheet, textureID, 64));
        }

        Self {
            wallTextures,
            gateSidewallTexturePair: TexturePair { litTextureID: 101, unlitTextureID: 102 }
        }
    }

    fn ExtractTextureFromSurface(sdlTextureCreator: &TextureCreator<WindowContext>, textureSheet: &Surface, textureID: i32, texturePitch: i32) -> Texture {
        let textureSheetPitch = 6;
        let textureX = ((textureID - 1) % textureSheetPitch ) * texturePitch;
        let textureY = ((textureID - 1) / textureSheetPitch ) * texturePitch;

        let mut extractedTextureSurface = Surface::new(texturePitch as u32, texturePitch as u32, PixelFormatEnum::ARGB8888).unwrap();
        let _ = textureSheet.blit(Rect::new(textureX, textureY, texturePitch as u32, texturePitch as u32), &mut extractedTextureSurface, Rect::new(0, 0, texturePitch as u32, texturePitch as u32));

        return sdlTextureCreator.create_texture_from_surface(&extractedTextureSurface).unwrap();
    }
}

pub fn LightTexture(rayCursor: &mut RayCursor, texturePair: &TexturePair) -> i32 {
    match rayCursor.GetWallType() {
        crate::utils::dda::wallType_t::HORIZONTAL => texturePair.unlitTextureID,
        crate::utils::dda::wallType_t::VERTICAL => texturePair.litTextureID,
        crate::utils::dda::wallType_t::CORNER => texturePair.unlitTextureID,
        crate::utils::dda::wallType_t::NONE => panic!()
    }
}

