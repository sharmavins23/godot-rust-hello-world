use godot::classes::{ISprite2D, Sprite2D};
use godot::obj::WithBaseField;
use godot::prelude::{Base, GodotClass, Vector2, godot_api, godot_print};

// ===== Definitions ===========================================================

/// Definition of our [`Player`] class, which inherits from [`Sprite2D`].
#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    /// The angular speed of the player, in radians per second.
    pub angular_speed: f32,

    /// The base [`Sprite2D`] instance that this [`Player`] is built upon.
    pub base: Base<Sprite2D>,

    /// The linear speed of the [`Player`], in pixels per second.
    pub speed: f32,
}

// ===== Implementations =======================================================

/// Implementation of the [`Player`] class, which inherits from [`Sprite2D`].
#[godot_api]
impl ISprite2D for Player {
    /// The init function is called when the [`Player`] node is created.
    ///
    /// # Parameters
    /// - `base`: Base [`Sprite2D`] instance that the [`Player`] is built upon.
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, worldy!"); // Prints to the Godot console

        Self {
            angular_speed: std::f32::consts::PI,
            base,
            speed: 400.0,
        }
    }

    /// Process called every physics frame.
    ///
    /// # Parameters
    /// - `self`: The [`Player`] instance.
    /// - `delta`: The time elapsed since the last physics frame, in seconds.
    fn physics_process(&mut self, delta: f32) {
        // Compute and apply our rotation
        let radians: f32 = self.angular_speed * delta;
        self.base_mut().rotate(radians);

        // Compute our velocity vector based on our current rotation and speed
        let rotation: f32 = self.base_mut().get_rotation();
        let velocity: Vector2 = Vector2::UP.rotated(rotation) * self.speed;
        self.base_mut().translate(velocity * delta);
    }
}
