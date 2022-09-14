mod camera_joint;
mod game;
mod grid;
mod player;
mod spinning_cube;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<spinning_cube::SpinningCube>();
    handle.add_class::<player::Player>();
    handle.add_class::<grid::Grid>();
    handle.add_class::<camera_joint::CameraJoint>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
