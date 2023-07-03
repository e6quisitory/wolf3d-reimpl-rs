# Wolfenstein 3D Clone in Rust
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

### Remaining
- [ ] Minimap

## Current Engine Demo
https://github.com/e6quisitory/wolf3d-clone-rs/assets/25702188/9ba6f8e1-b113-4c07-8c96-c80e9bdc7c87

## Issues/Dev Notes
- Things that don’t need to be pub shouldn’t be
- Implement proper config file for things like full screen, window width and height, player initial location + viewDir
- Making fov higher seems to increase the height of blocks. Might need to add this into propr. constant

## Later Goals
- [ ] Port to [WebAssembly](https://rustwasm.github.io/docs/book)