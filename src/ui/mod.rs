pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::CalculationState;

// UIプラグイン
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CalculationState>()
            .add_systems(Startup, systems::setup_ui)
            .add_systems(
                Update,
                (
                    systems::button_system,
                    systems::number_display_system,
                    systems::calculation_display_system,
                ),
            );
    }
}
