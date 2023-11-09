# Wolfenstein 3D Re-implementation

<img src="https://github.com/e6quisitory/wolf3d-clone-rs/assets/25702188/8a718a7b-a9b0-4114-8764-75b5e1ef89c9" width=350/>

An in-progress, from-scrach, re-implementation of the FPS-genre pioneering game, [_Wolfenstein 3D_](https://en.wikipedia.org/wiki/Wolfenstein_3D), released by id Software in 1992, in Rust.
The goal is to re-implement all of the game's original functionality, as well as potentially add custom features like networked multiplayer.

## Current Engine Demo
https://github.com/e6quisitory/wolf3d-clone-rs/assets/25702188/c84cd321-ca95-4f4e-8ccf-91a684fb45e7

### Remaining Features to Implement
- [ ] Enemy AI
- [ ] More weapons
- [ ] WAD parser
- [ ] Minimap
- [ ] Networked multiplayer _(aspirational)_
- [ ] Port to [WebAssembly](https://rustwasm.github.io/docs/book)

### Completed
- [x] Raycasting
- [x] Texture mapping
- [x] Sprites (objects & enemies)
- [x] Doors
- [x] Controls (WASD, arrow keys, mouse) 
- [x] Custom map creation
- [x] Player-wall, player-objetc collision detection
- [x] Moving enemies
- [x] Shooting enemies

## Build Instructions
Assuming you have `rustc` and `cargo` installed (through `rustup`, most likely), as well as [SDL2](https://github.com/libsdl-org/SDL/releases), clone this repo, then, from its root, run the following command.
```
cargo run --release
```
## Controls
- `W` `A` `S` `D` to move around
- Mouse (or trackpad) to look around
- Left mouse click to shoot
- `Space` to open doors
- `~` to unlock/relock mouse from game window
- `Esc` to quit game

## Background, Technical Details, Current State of Affairs
This project started as an effort to teach myself hardcore systems-level programming by working on a cool but challenging project.

As it turns out, games are pretty technical beasts under the hood. So I thought implementing a game from scratch would be quite the challenge to put on my plate.
However, it soon became apparent that a modern triple-A game takes hundreds of people and millions of lines of code to pull off. So I couldn't do that as a beginner.

But the great thing with games (or really any technology, I suppose) is that you can always go back in time. Re-implementing a game like _Wolfenstein 3D_ from the early 90's—a historial game, given that it essentially pioneered the first-person-shooter genre—seemed like an appropriate challenge; complex and tough, but certainly doable by one person.

I started off the project in C++ [(repo here)](https://github.com/e6quisitory/wolf3d-clone). A few months in, after I had the renderer working, I decided to learn Rust, and, as a first project to teach myself the language, I ported over all the code to Rust. It turned out to be such a positive experience that I decided to move the project over to Rust permanently.

The core engine that powers the game is all written from scratch. The things not written from scrach are opening a window, putting pixels into it, reading mouse and keyboard inputs, managing textures, GPU-accelerated texture scaling, and doing all this in a cross-platform manner. To do all this for me, I'm using the excellent [SDL](https://github.com/libsdl-org/SDL) library. Doing all these tasks listed from scratch as well instead of using a library to accomplish them would be an interesting challenge, but I feel that would stray from the scope of the project.

The current state of the project can be summarized as transitioning past just being a renderer and into an actual game. When I started the project, I though the renderer was the hardest part. But implementing it ironically turned out to be the tip of the complexity iceberg. Adding in enemies, a general animation system, enemy AI, efficient asset management—implementing all these things alongside the renderer, while keeping the project clean and organized, is turning out to be the real challenge I have ahead of me.

Certainly the most important lesson I've learned thus far is that implementing signular features or subsystems is not that hard. Implementation is not hard in general. What's _actually_ hard is implementing many subsystems and having them talk to each other and work together as part of a larger system in a clean, efficient, and maintainable way. I never in a million years would've thought that program design/architecture is a (much) harder problem than actually implementing the program (i.e. writing the raw code and routines that implement core features), but it is certainly the case.

To conclude, the latest development is that a general animation system is in place, though it needs some refactoring. After that will come the grand challenge of adding in enemy AI with many types of enemies, as well as multiple weapons. Once that's in, the project will be largely complete as we'll have a playable game. Implementing a WAD parser would be the next logical step, so that original levels of the game could be played. And finally, there's networked multiplayer. I'm stoked to add that in. It'll be my first foray into computer networking.

I'm also thinking of cleaning up the current code a bit more and writing documentation for how everything works. The purpose behind this is so that I can bring more people onboard the project.

## Credits & Disclaimer
As this is a re-implementation of a classic game, I'd like to state the obvious and say that this isn't "my" game. It is id Software's game, released by them in 1992. The entire concept of the game, the design, the mechanics, all of the artwork, is most definitely not my creation, it is theirs.

My creation and original work in this project lies purely in the technical realm of writing the code to replicate the workings of the game (hence, a re-implementation); all the logic and algorithms required. This is a purely educational pursuit for myself, I'm not trying to make any money off this (and if I tried, I'd have a lawsuit on my hands), I just want that sweet sweet technical and programming street cred.

I found all the original artwork from the game [here](https://www.spriters-resource.com/pc_computer/wolfenstein3d/).
