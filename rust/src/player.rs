use crate::input::InputHandler;
use godot::classes::{ISprite2D, Sprite2D};
use godot::obj::WithBaseField;
use godot::prelude::{Base, GodotClass, Vector2, godot_api, godot_print};

// ===== Definitions ===========================================================

/// Definition of our [`Player`] class, which inherits from [`Sprite2D`].
#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
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

        Self { base, speed: 400.0 }
    }

    /// Process called every physics frame.
    ///
    /// # Parameters
    /// - `self`: The [`Player`] instance.
    /// - `delta`: The time elapsed since the last physics frame, in seconds.
    fn physics_process(&mut self, delta: f32) {
        // Get movement direction
        let direction: Vector2 = InputHandler::get_movement_direction();

        // Apply movement in that direction
        let velocity: Vector2 = direction * self.speed;
        self.base_mut().translate(velocity * delta);

        // Rotate based on movement direction
        if direction != Vector2::ZERO {
            self.base_mut().set_rotation(direction.angle());
        }
    }
}
