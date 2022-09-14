use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct CameraJoint {
    #[property(default = 0.05)]
    mouse_sensivity: f32,
}

#[methods]
impl CameraJoint {
    fn new(_owner: &Spatial) -> Self {
        Self {
            mouse_sensivity: 0.05,
        }
    }

    #[method]
    fn _ready(&self, #[base] owner: &Spatial) {
        godot_print!("Mouse sensivity is {}", self.mouse_sensivity);

        owner.set_as_toplevel(true);
        let input = Input::godot_singleton();
        input.set_mouse_mode(Input::MOUSE_MODE_CAPTURED);
    }

    #[method]
    fn _unhandled_input(&self, #[base] owner: &Spatial, event: Ref<InputEvent>) {
        let event = unsafe { event.assume_safe() };

        if let Some(event) = event.cast::<InputEventMouseMotion>() {
            let mut degrees = owner.rotation_degrees();

            degrees.x -= event.relative().y * self.mouse_sensivity;
            degrees.x = degrees.x.max(-90.0).min(30.0);

            degrees.y -= event.relative().x * self.mouse_sensivity;
            degrees.y = degrees.y.max(0.0).min(360.0);

            owner.set_rotation_degrees(degrees);
        }
    }
}
