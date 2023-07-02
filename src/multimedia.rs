use std::collections::HashMap;
use std::hash::Hash;
use sdl2::{EventPump, pixels};
use sdl2::{
    image,
    video::Window
};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::{Surface};
use sdl2::video::WindowContext;
use crate::tiles::TextureHandle;
use crate::utils::conventions::TEXTURE_PITCH;
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
        let assets = Assets::New(&sdlTextureCreator);

        sdlContexts.sdlContext.mouse().set_relative_mouse_mode(true);

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
    pub projPlaneDist: f64,
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

        let projPlaneDist = (windowWidth as f64 / 2.0)/DegreesToRadians(fov/2.0).tan();
        
        // Render height proportionality constant ; takes into account screen aspect ratio
        let renderHeightProprConst = 1.15 * (windowWidth as f64) / ((16.0 / 9.0) * (fov / 72.0));  

        RenderParams {
            fov,
            projPlaneDist,
            castingRayAngles,
            renderHeightProprConst
        }
    }
}

pub struct Assets {
    textureSheets: HashMap<TextureType, TextureSheet>,
}

impl Assets {
    pub fn New(sdlTextureCreator: &TextureCreator<WindowContext>) -> Self {
        let wall_TS    = TextureSheet::New(sdlTextureCreator,  "assets/wall_textures.bmp", 6, 110, false);
        let object_TS  = TextureSheet::New(sdlTextureCreator,  "assets/objects.bmp", 5, 50, true);
        let guard_TS   = TextureSheet::New(sdlTextureCreator,  "assets/guard.bmp", 8, 51, true);
        let officer_TS = TextureSheet::New(sdlTextureCreator,  "assets/officer.bmp", 8, 51, true);
        let SS_TS      = TextureSheet::New(sdlTextureCreator,  "assets/SS.bmp", 8, 51, true);

        let mut textureSheets: HashMap<TextureType, TextureSheet> = HashMap::new();
            textureSheets.insert(TextureType::WALL, wall_TS);
            textureSheets.insert(TextureType::OBJECT, object_TS);
            textureSheets.insert(TextureType::GUARD, guard_TS);
            textureSheets.insert(TextureType::OFFICER, officer_TS);
            textureSheets.insert(TextureType::SS, SS_TS);

        Self {
            textureSheets
        }
    }

    pub fn GetTexture(&self, textureHandle: TextureHandle) -> &Texture {
        &self.textureSheets[&textureHandle.textureType].textures[(textureHandle.ID-1) as usize]
    }

}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum TextureType {
    WALL,
    OBJECT,
    GUARD,
    OFFICER,
    SS
}

struct TextureSheet {
    filename: String,
    pitch: i32,
    pub textures: Vec<Texture>
}

impl TextureSheet {
    fn New(sdlTextureCreator: &TextureCreator<WindowContext>, filename: &str, sheetPitch: i32, numTextures: i32, colorKey: bool) -> Self {
        let mut textures: Vec<Texture> = Vec::new();
        let mut sheetSurface = Surface::load_bmp(filename).unwrap();

        if colorKey {
            let _ = sheetSurface.set_color_key(true, pixels::Color{r: 152, g: 0, b: 136, a: 255}).unwrap();
        }

        for textureID in 1..=numTextures {
            textures.push ({
                let textureX = ((textureID - 1) % sheetPitch) * TEXTURE_PITCH as i32;
                let textureY = ((textureID - 1) / sheetPitch) * TEXTURE_PITCH as i32;
        
                let mut extractedTextureSurface = Surface::new(TEXTURE_PITCH, TEXTURE_PITCH, PixelFormatEnum::ARGB8888).unwrap();
                let _ = sheetSurface.blit(Rect::new(textureX, textureY, TEXTURE_PITCH, TEXTURE_PITCH), &mut extractedTextureSurface, Rect::new(0, 0, TEXTURE_PITCH, TEXTURE_PITCH));
        
                sdlTextureCreator.create_texture_from_surface(&extractedTextureSurface).unwrap()
        });
        }
        
        Self {
            filename: filename.to_string(),
            pitch: sheetPitch,
            textures
        }
    }
}

pub fn LightTexture(rayCursor: &mut RayCursor, litTexture: TextureHandle, unlitTexture: TextureHandle) -> TextureHandle {
    match rayCursor.GetWallType() {
        crate::utils::dda::wallType_t::HORIZONTAL => unlitTexture,
        crate::utils::dda::wallType_t::VERTICAL => litTexture,
        crate::utils::dda::wallType_t::CORNER => unlitTexture,
        crate::utils::dda::wallType_t::NONE => panic!()
    }
}
