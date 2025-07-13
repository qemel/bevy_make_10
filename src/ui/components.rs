use bevy::prelude::*;

// UIコンポーネント定義

// 数字表示用のコンポーネント
#[derive(Component)]
pub struct NumberDisplay {
    pub value: u32,
    pub index: usize, // 0-3の数字のインデックス
}

// 演算ボタン用のコンポーネント  
#[derive(Component)]
pub struct OperatorButton {
    pub operator: char, // '+', '-', '*', '/'
}

// 現在の計算式表示用のコンポーネント
#[derive(Component)]
pub struct ExpressionDisplay;

// リセットボタン用のコンポーネント
#[derive(Component)]  
pub struct ResetButton;

// スコア表示用のコンポーネント
#[derive(Component)]
pub struct ScoreDisplay;

// ゲーム画面のメインコンテナ
#[derive(Component)]
pub struct GameScreenContainer;