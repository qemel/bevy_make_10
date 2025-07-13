mod game;
mod ui;

use bevy::prelude::*;
use game::{GameNumbers};
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Make 10 Game".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UIPlugin)
        .insert_resource(GameNumbers::new()) // ゲーム用のリソースとして数字を追加
        .run();
}
