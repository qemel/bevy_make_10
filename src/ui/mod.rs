pub mod components;
pub mod systems;

use bevy::prelude::*;

// UIプラグイン
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::setup_ui)
            .add_systems(Update, (
                systems::button_system,
                systems::number_display_system,
            ));
    }
}