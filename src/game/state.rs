//! ゲーム状態管理

use bevy::prelude::*;

/// ゲームの状態を表すenum
#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub enum GameState {
    /// ゲーム中
    #[default]
    Playing,
    /// ステージクリア
    StageClear,
    /// ゲームオーバー
    GameOver,
}

/// ゲーム進行状態を管理するリソース
#[derive(Resource)]
pub struct GameProgress {
    pub current_stage: u32,
    pub score: u32,
    pub stages_cleared: u32,
}

impl Default for GameProgress {
    fn default() -> Self {
        Self {
            current_stage: 1, // ステージ1から開始
            score: 0,
            stages_cleared: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state_is_playing() {
        // テスト: デフォルト状態がPlayingであることを確認
        let state = GameState::default();
        assert_eq!(state, GameState::Playing);
    }
}
