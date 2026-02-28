use godot::prelude::{ExtensionLibrary, gdextension};

mod player;

// ===== Definitions ===========================================================

/// Definition of the [`ExtensionLibrary`] loading our scripts into Godot.
struct HelloWorldExtension;

// ===== Implementations =======================================================

/// Implementation of the [`ExtensionLibrary`] loading our scripts into Godot.
#[gdextension]
unsafe impl ExtensionLibrary for HelloWorldExtension {}
