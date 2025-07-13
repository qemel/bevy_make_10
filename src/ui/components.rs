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

// 計算結果表示用のコンポーネント
#[derive(Component)]
pub struct ResultDisplay;

// リセットボタン用のコンポーネント
#[derive(Component)]
pub struct ResetButton;

// スコア表示用のコンポーネント
#[derive(Component)]
pub struct ScoreDisplay;

// ゲーム画面のメインコンテナ
#[derive(Component)]
pub struct GameScreenContainer;

// 計算状態を管理するリソース
#[derive(Resource, Default)]
pub struct CalculationState {
    pub expression: String,
    pub result: Option<f64>,
    pub selected_numbers: Vec<usize>, // 選択された数字のインデックス
    pub operators: Vec<char>,         // 使用された演算子
}

// ステージクリアポップアップ関連のコンポーネント
#[derive(Component)]
pub struct StageClearPopup;

#[derive(Component)]
pub struct NextStageButton;

#[derive(Component)]
pub struct PopupOverlay;
