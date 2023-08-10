# Wolfenstein 3D Clone in Rust

<img src="https://github.com/e6quisitory/wolf3d-clone-rs/assets/25702188/0cb948af-8d73-469a-8786-c6615d95caf6" width=300/>

I've previously [implemented](https://github.com/e6quisitory/wolf3d-clone) a Wolfenstein 3D renderer in C++. Now, I've decided it's time to hop on the Rust bandwagon. So, as a first project, to teach myself the language, I'm porting over the project to Rust.

### Completed
- [x] Raycasting
- [x] Texture mapping
- [x] Custom map loading from CSV
- [x] Controls (WASD & arrow keys)
- [x] Wall collision detection
- [x] Doors
- [x] Mouse to look around
- [x] Sprites (objects & enemies)
- [x] Moving enemies

### Remaining
- [ ] Minimap

## Current Engine Demo
https://github.com/e6quisitory/wolf3d-clone-rs/assets/25702188/af591bf3-c4cc-4f11-87ce-a71f3f310d98

## Build Instructions
Assuming you have `rustc` and `cargo` installed (through `rustup`, most likely), clone this repo, then, from its root, run the following command.
```
cargo run --release
```
## Issues/Dev Notes
- Things that don’t need to be pub shouldn’t be
- Implement proper config file for things like full screen, window width and height, player initial location + viewDir
- Making fov higher seems to increase the height of blocks. Might need to add this into propr. constant

## Later Goals
- [ ] Weapons
- [ ] Enemy AI
- [ ] Port to [WebAssembly](https://rustwasm.github.io/docs/book)

## Credits
All wall and sprite textures + the logo at the top of this README are from the official Wolfenstein 3D artwork that shipped in the game back in 1992. I found them [here](https://www.spriters-resource.com/pc_computer/wolfenstein3d/).
