//! ゲーム状態管理

/// ゲームの状態を表すenum
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    /// ゲーム中
    Playing,
    /// ステージクリア
    StageClear,
    /// ゲームオーバー
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Playing
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