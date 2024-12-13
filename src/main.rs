use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use specs::{System, World, WorldExt};

struct GameState;

impl GameState {
    fn new() -> Self {
        GameState
    }

    fn update(&mut self) {
        // The place to update the game logic, such as updating game state, handling input, etc.
        println!("Updating game state...");
    }
}

fn main() {
    // Set up the event loop
    let event_loop = EventLoop::new();

    // Create the window
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Initialize the game state
    let mut game_state = GameState::new();

    // Set up the game world using specs ECS
    World::new();
    // Add your entities, components, etc. here

    // Main game loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Closing window...");
                    *control_flow = ControlFlow::Exit; // Stop the event loop
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        println!("Escape key pressed!");
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Update the game state
                game_state.update();
                // Execute game logic here, such as handling input, updating object positions, etc.
            }
            _ => {}
        }
    });
}
