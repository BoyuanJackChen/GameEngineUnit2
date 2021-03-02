use pixels::{Pixels, SurfaceTexture};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use image;

// Whoa what's this?
// Mod without brackets looks for a nearby file.
mod screen;
// Then we can use as usual.  The screen module will have drawing utilities.
use screen::Screen;
// Collision will have our collision bodies and contact types
// mod collision;
// Lazy glob imports
// use collision::*;
// Texture has our image loading and processing stuff
mod texture;
use texture::Texture;
// Animation will define our animation datatypes and blending or whatever
mod animation;
use animation::Animation;
// Sprite will define our movable sprites
mod sprite;
// Lazy glob import, see the extension trait business later for why
use sprite::*;
// And we'll put our general purpose types like color and geometry here:
mod types;
use types::*;
mod tiles;
use tiles::*;
// Now this main module is just for the run-loop and rules processing.
struct GameState {
    // What data do we need for this game?  Wall positions?
    // Colliders?  Sprites and stuff?
    animations: Vec<Rc<Animation>>,
    textures: Vec<Rc<Texture>>,
    //tiles: Vec<Rc<Tileset>>,
    tilemap:Vec<Tilemap>,
    sprites: Vec<Sprite>,
    scroll_pos:Vec2i
}
// seconds per frame
const DT: f64 = 1.0 / 60.0;

const WIDTH: usize = 125;
const HEIGHT: usize = 125;
const DEPTH: usize = 4;
const POSITION:Vec2i=Vec2i(0,0);

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Tilemap")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };
    let frames1 =vec![
        Rect {
        x: 0,
        y: 0,
        w: 26,
        h: 36,
    }, Rect {
        x: 0,
        y: 36,
        w: 26,
        h: 36,
    },
    Rect {
        x: 0,
        y: 72,
        w: 26,
        h: 36,
    }
    ];
    let looping = true;
    let time_per = vec![5,5,5];
    let time_per2 = vec![10,10,10];
    let sprite_tex = Rc::new(Texture::with_file(Path::new("content/sprites.png")));
    let anim1=Rc::new(Animation::new(time_per, frames1, looping));
    let tex = Rc::new(Texture::with_file(Path::new("content/tileset.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
        ],
        &tex,
    ));
    let map1 = Tilemap::new(
        Vec2i(0, 0),
        (8, 8),
        &tileset,
        vec![
            1, 1, 1, 1, 1, 1, 1, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 

        ],
    );
    let map2 = Tilemap::new(
        Vec2i(128, 0),
        (8, 8),
        &tileset,
        vec![
            1, 1, 1, 1, 1, 1, 1, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 

        ],
    );
    let map3 = Tilemap::new(
        Vec2i(256, 0),
        (8, 8),
        &tileset,
        vec![
            1, 1, 1, 1, 1, 1, 1, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 0, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 

        ],
    );
    let mut state = GameState {
        // initial game state...
        animations: vec![Rc::clone(&anim1)],
        sprites: vec![Sprite::new(
            &sprite_tex,
            &anim1,
            Vec2i(0, 0),
            Direction::Up,
            0,
        )],
        textures: vec![tex],
        tilemap:vec![map1,map2,map3],         
        scroll_pos:Vec2i(0,0)
    };
    // How many frames have we simulated?
    let mut frame_count: usize = 0;
    // How many unsimulated frames have we saved up?
    let mut available_time = 0.0;
    // Track beginning of play
    let start = Instant::now();
    // Track end of the last frame
    let mut since = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH, state.scroll_pos);
            screen.clear(Rgba(0, 0, 0, 0));

            draw_game(&state, &mut screen, frame_count);

            // Flip buffers
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Rendering has used up some time.
            // The renderer "produces" time...
            available_time += since.elapsed().as_secs_f64();
        }
        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window if needed
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
        }
        // And the simulation "consumes" it
        while available_time >= DT {
            // Eat up one frame worth of time
            available_time -= DT;

            update_game(&mut state, &input, frame_count);

            // Increment the frame counter
            frame_count += 1;
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}

fn draw_game(state: &GameState, screen: &mut Screen, frame_count:usize) {
    // Call screen's drawing methods to render the game state
    screen.clear(Rgba(232, 232, 232, 1));
    
    for t in state.tilemap.iter(){
        t.draw(screen);
    }
    //screen.line(Vec2i(0, 150), Vec2i(300, 200), Rgba(46, 49, 49, 1));
    for s in state.sprites.iter() {
        screen.draw_sprite(s);
    }   
 }


fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    //Player control goes here
    if input.key_held(VirtualKeyCode::Right) {
        if state.scroll_pos.0 < 256{
            state.sprites[0].position.0 += 3;
            state.scroll_pos.0+=2;
        }      
    }
    if input.key_held(VirtualKeyCode::Left) {
        if state.scroll_pos.0 > 0{
            state.sprites[0].position.0 -= 3;
            state.scroll_pos.0-=2;
        }
    }
    if input.key_held(VirtualKeyCode::Up) {
        state.sprites[0].position.1 -= 2;
    }
    if input.key_held(VirtualKeyCode::Down) {
        state.sprites[0].position.1 += 2;
    }
    for s in state.sprites.iter_mut(){
        s.tick();
    }  
    
    // Update player position

    // Detect collisions: Generate contacts

    // Handle collisions: Apply restitution impulses.

    // Update game rules: What happens when the player touches things?
}

