# Slugcat Race Tests (SRT)
"this peaks"
> -Vulkan
---
Slugcat Race Tests (SRT) is inspired from [Horse Race Tests](https://x.com/snakesandrews/status/1915799331220684835) where Rain World slugcats race to collect food.

Built in Rust+Raylib, SRT is an open-source program that allows you to:
- Create your own maps for races
- Replace, remove, or add more racers
- Mod all external sound and image

The goal for each race is to get the food. First one to get it wins!

# Features
- Pixel-perfect collision between slugcats and walls
- Modular maps
- Fast performence (thanks to raylib and rust!)
- Multi-threaded slugcat updates
- Object-oriented (i think)
- Easy way to add more slugcat racers
- Slugcat

# Usage

# Download
Windows and Linux versions avaliable on [itch.io](https://mechinsam.itch.io/slugcat-race-tests)
Both 64 bit.

# Compiling
1. Download Rust
2. Clone the repo with the terminal command `git clone [[[` or use Github Desktop (GUI)
3. Go into the cloned folder and open your terminal
4. Run the command `cargo run`

This will automatically build and run the program.
The program should be in the target/debug sub folder if you dont want to run that command every time (Make sure to copy the DATA folder to that directory as well)

# Modding
## Slugcats
Slugcat images are located in DATA/racers
There are two folders:
- sprites - folder for in-race sprites
- win - folder for win images

There are template files for both if you want to use those!

To add a slugcat to the game, just make a new sprite. The name of the slugcat will be pulled from the filename.
A win image is not required as a default one will be used but feel free to add one!

## Maps
Located in DATA/maps, a map folder must contain
- bg.png - background image
- col_map.png - a collision map in the form of an image. Red parts are where collisions can occur, and regions with below 25% opcaity (idealy 0%), are free zones. Borders of the image do not need their collision highlighted
- food.png - Food sprite to use for that level. Keep it roughly 200x400 to not affect scaling

**Your bg.png and col_map.png MUST BE 1024x768.**
If you don't like this size, you can change the SCREEN_WIDTH & SCREEN_HEIGHT variables in main.rs and recompile but this will make pre-packaged maps incompatiable
