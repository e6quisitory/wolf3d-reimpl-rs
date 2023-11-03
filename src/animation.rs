use crate::tiles::TextureHandle;

#[derive(Clone)]
pub struct AnimationReel {
    pub textures: Vec<TextureHandle>,
    pub currTextureIndex: u32,

    timeInterval: f64,
    perFrameTimeIncrement: f64,
    timer: f64,

    loopbackClipIndex: Option<u32>
}

impl AnimationReel {
    pub fn New(textures: Vec<TextureHandle>, timeInterval: f64, perFrameTimeIncrement: f64, loopbackClipIndex: Option<u32>) -> Self {
        Self {
            textures,
            currTextureIndex: 0,
            timeInterval,
            perFrameTimeIncrement,
            timer: 0.0,
            loopbackClipIndex
        }
    }

    fn Reset(&mut self) {
        self.currTextureIndex = 0;
        self.timer = 0.0;
    }

    fn IncrementTexture(&mut self) {
        self.currTextureIndex += 1;
        self.timer = 0.0;
    }

    pub fn Update(&mut self) -> Option<u32> {
        // If timer is up
        if self.timer >= self.timeInterval {
            // If on last texture in reel
            if self.currTextureIndex == (self.textures.len() - 1) as u32 {
                self.Reset();
                // If there is a loopback clip to go to, return that clip index
                if self.loopbackClipIndex.is_some() {
                    self.loopbackClipIndex
                // If no loopback clip to go to, keep reel playing
                } else {
                    None
                }
            // If not on last texture in reel, increment texture
            } else {
                self.IncrementTexture();
                None
            }
        // If timer is not up
        } else {
            self.timer += self.perFrameTimeIncrement;
            None
        }
    }
}
#[derive(Clone)]
pub enum AnimationClip {
    STATIC(TextureHandle),
    REEL(AnimationReel)
}

#[derive(Clone)]
pub struct AnimationMagazine {
    pub clips: Vec<AnimationClip>,
    pub currClipIndex: u32
}

impl AnimationMagazine {
    pub fn New(clips: Vec<AnimationClip>, currClipIndex: u32) -> Self {
        Self {
            clips,
            currClipIndex,
        }
    }

    pub fn Update(&mut self) {
        match &mut self.clips[self.currClipIndex as usize] {
            AnimationClip::STATIC(_) => {
                // do nothing
            },
            AnimationClip::REEL(reel) => {
                if let Some(loopbackClipIndex) = reel.Update() {
                    self.currClipIndex = loopbackClipIndex;
                }
            }
        }
    }

    pub fn GetCurrTexture(&self) -> TextureHandle {
        match &self.clips[self.currClipIndex as usize] {
            AnimationClip::STATIC(texture) => {
                *texture
            }
            AnimationClip::REEL(reel) => {
                reel.textures[reel.currTextureIndex as usize]
            }
        }
    }

    pub fn SwitchClipIndexWithTimeCopy(&mut self, clipIndexToSwitchTo: usize) {
        let backupTextureIndex;
        let backupTimer;
        if let AnimationClip::REEL(oldReel) = &mut self.clips[self.currClipIndex as usize] {
            backupTextureIndex = oldReel.currTextureIndex;
            backupTimer = oldReel.timer;
            oldReel.Reset();
        } else {
            panic!()
        }
        self.currClipIndex = clipIndexToSwitchTo as u32;
        if let AnimationClip::REEL(newReel) = &mut self.clips[self.currClipIndex as usize] {
            newReel.currTextureIndex = backupTextureIndex;
            newReel.timer = backupTimer;
        }
    }
}
