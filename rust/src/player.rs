use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    #[property(default = 7.0)]
    speed: f32,
    #[property(default = 20.0)]
    jump_strength: f32,
    gravity: f32,
    velocity: Vector3,
    snap_vector: Vector3,
    spring_arm: Option<Ref<SpringArm>>,
    // player_model: Spatial,
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Self {
            speed: 7.0,
            jump_strength: 20.0,
            gravity: 50.0,
            velocity: Vector3::ZERO,
            snap_vector: Vector3::DOWN,
            spring_arm: Some(SpringArm::new().into_shared()),
        }
    }

    #[method]
    fn _ready(&mut self, #[base] owner: &KinematicBody) {
        let spring_arm = owner
            .get_node("SpringArm")
            .expect("this node must have a child with the path `SpringArm`");
        let spring_arm = unsafe { spring_arm.assume_safe() };
        let spring_arm = spring_arm
            .cast::<SpringArm>()
            .expect("child must be of type 'SpringArm'");
        self.spring_arm = Some(spring_arm.claim());
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody, delta: f32) {
        let input = Input::godot_singleton();
        let mut direction = Vector3::ZERO;

        direction.x = input.get_action_strength("right", false) as f32
            - input.get_action_strength("left", false) as f32;
        direction.z = input.get_action_strength("back", false) as f32
            - input.get_action_strength("forward", false) as f32;

        let spring_arm = unsafe { self.spring_arm.unwrap().assume_safe() };

        direction = direction
            .rotated(Vector3::UP, spring_arm.rotation().y)
            .normalized();

        // TODO: optimize check for NaNs
        if !direction.x.is_finite() {
            direction.x = 0.0;
        }
        if !direction.y.is_finite() {
            direction.y = 0.0;
        }
        if !direction.z.is_finite() {
            direction.z = 0.0;
        }

        self.velocity.x = direction.x * self.speed;
        self.velocity.z = direction.z * self.speed;
        self.velocity.y -= self.gravity * delta;

        let just_landed = owner.is_on_floor() && self.snap_vector == Vector3::ZERO;
        let is_jumping = owner.is_on_floor() && input.is_action_just_pressed("jump", false);

        if is_jumping {
            self.velocity.y = self.jump_strength;
            self.snap_vector = Vector3::ZERO;
        } else if just_landed {
            self.snap_vector = Vector3::DOWN;
        }
        self.velocity = owner.move_and_slide_with_snap(
            self.velocity,
            self.snap_vector,
            Vector3::UP,
            false,
            4,
            0.785398,
            true,
        );
        godot_dbg!(":?", self.velocity);

        // TODO: Rotate the model
        // if self.velocity.length() > 0.2
    }

    #[method]
    fn _process(&self, #[base] owner: &KinematicBody, _delta: f32) {
        unsafe {
            self.spring_arm
                .unwrap()
                .assume_safe()
                .set_translation(owner.translation());
        }
    }
}
