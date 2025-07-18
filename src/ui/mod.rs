pub mod components;
#[cfg(test)]
mod expression_tests;
pub mod systems;

use crate::game::state::{GameProgress, GameState};
use bevy::prelude::*;
use components::CalculationState;

// UIプラグイン
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CalculationState>()
            .init_resource::<CalculationState>()
            .init_resource::<GameState>()
            .init_resource::<GameProgress>()
            .add_systems(Startup, systems::setup_ui)
            .add_systems(
                Update,
                (
                    systems::button_system,
                    systems::number_display_system,
                    systems::calculation_display_system,
                    systems::stage_clear_detection_system,
                    systems::popup_system,
                    systems::game_info_display_system,
                ),
            );
    }
}
