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

### Remaining
- [ ] Sprites (objects & enemies)
- [ ] Minimap

## Current Engine Demo
https://github.com/e6quisitory/wolf3d-clone-rs/assets/25702188/ac4d754d-5bd4-4b00-ab79-9f08909f63a0

## Issues/Dev Notes
- Things that don’t need to be pub shouldn’t be
- Clean up texture structs and enums, quite confusing right now
- New CSV parser to take into account different tile types
- Implement sprite rendering logic; currently only have wall & door render logic
    - Will mean having tiles that store sprites, like doors storing sprites
    - When an enemy dies, they leave behind ammo. So will need dead body sprite + collectible inside one tile
    - Multiple dead enemies inside one tile
    - If enemy in doorway, do not close door
- Implement proper config file for things like full screen, window width and height
- Making fov higher seems to increase the height of blocks. Might need to add this into propr. constant

## Later Goals
- [ ] Port to [WebAssembly](https://rustwasm.github.io/docs/book)