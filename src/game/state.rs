//! ゲーム状態管理

/// ゲームの状態を表すenum
#[derive(Debug, Clone, PartialEq, Default)]
pub enum GameState {
    /// ゲーム中
    #[default]
    Playing,
    /// ステージクリア
    StageClear,
    /// ゲームオーバー
    GameOver,
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
