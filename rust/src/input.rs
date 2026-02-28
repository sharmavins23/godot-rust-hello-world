use godot::classes::Input;
use godot::obj::Singleton;
use godot::prelude::Vector2;

// ===== Definitions ===========================================================

/// Definition of keyboard input handler for the [`Player`]'s movement.
pub struct InputHandler;

// ===== Implementations =======================================================

/// Implementation of the keyboard input handler for the [`Player`]'s movement.
impl InputHandler {
    /// Returns the movement direction as a normalized vector.
    pub fn get_movement_direction() -> Vector2 {
        let input = Input::singleton();

        // Mapping of input actions to movement directions
        let actions: [(&str, Vector2); 4] = [
            ("ui_up", Vector2::UP),
            ("ui_down", Vector2::DOWN),
            ("ui_left", Vector2::LEFT),
            ("ui_right", Vector2::RIGHT),
        ];

        // Compute the sum of all input direction vectors
        let direction: Vector2 = actions
            .iter()
            .filter(|(action, _)| input.is_action_pressed(*action))
            .fold(Vector2::ZERO, |sum_vector, (_, direction)| {
                sum_vector + *direction
            });

        // Normalize the direction vector to prevent faster diagonal movement
        if direction != Vector2::ZERO {
            direction.normalized()
        } else {
            Vector2::ZERO
        }
    }
}
