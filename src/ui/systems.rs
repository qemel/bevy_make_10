use bevy::prelude::*;
use crate::game::GameNumbers;
use super::components::*;

// UI初期化システム
pub fn setup_ui(mut commands: Commands, game_numbers: Res<GameNumbers>) {
    // カメラの作成
    commands.spawn(Camera2d);

    // メインUIコンテナの作成
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            GameScreenContainer,
        ))
        .with_children(|parent| {
            // タイトル
            parent.spawn((
                Text::new("Make 10 Game"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // 数字表示エリア
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                ))
                .with_children(|numbers_parent| {
                    // ゲームの数字を表示
                    for i in 0..4 {
                        let digit_value = game_numbers.digits[i];
                        numbers_parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.5, 0.7)),
                                NumberDisplay { value: digit_value as u32, index: i },
                            ))
                            .with_children(|button_parent| {
                                button_parent.spawn((
                                    Text::new(digit_value.to_string()),
                                    TextFont {
                                        font_size: 32.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                    }
                });

            // 演算ボタンエリア
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        column_gap: Val::Px(15.0),
                        ..default()
                    },
                ))
                .with_children(|operators_parent| {
                    let operators = ['+', '-', '*', '/'];
                    for &op in &operators {
                        operators_parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(60.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.5, 0.3, 0.7)),
                                OperatorButton { operator: op },
                            ))
                            .with_children(|button_parent| {
                                button_parent.spawn((
                                    Text::new(op.to_string()),
                                    TextFont {
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                    }
                });

            // 計算式表示エリア
            parent.spawn((
                Text::new("計算式: "),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                ExpressionDisplay,
            ));

            // リセットボタン
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.3, 0.3)),
                    ResetButton,
                ))
                .with_children(|button_parent| {
                    button_parent.spawn((
                        Text::new("リセット"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

// ボタンのインタラクションシステム
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&NumberDisplay>, Option<&OperatorButton>, Option<&ResetButton>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, number_display, operator_button, reset_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(number) = number_display {
                    println!("数字ボタンが押されました: {} (index: {})", number.value, number.index);
                } else if let Some(operator) = operator_button {
                    println!("演算ボタンが押されました: {}", operator.operator);
                } else if reset_button.is_some() {
                    println!("リセットボタンが押されました");
                }
                
                // 押下時の色変更
                *color = Color::srgb(0.8, 0.8, 0.8).into();
            }
            Interaction::Hovered => {
                // ホバー時の色変更
                if number_display.is_some() {
                    *color = Color::srgb(0.4, 0.6, 0.8).into();
                } else if operator_button.is_some() {
                    *color = Color::srgb(0.6, 0.4, 0.8).into();
                } else if reset_button.is_some() {
                    *color = Color::srgb(0.7, 0.4, 0.4).into();
                }
            }
            Interaction::None => {
                // 通常時の色に戻す
                if number_display.is_some() {
                    *color = Color::srgb(0.3, 0.5, 0.7).into();
                } else if operator_button.is_some() {
                    *color = Color::srgb(0.5, 0.3, 0.7).into();
                } else if reset_button.is_some() {
                    *color = Color::srgb(0.6, 0.3, 0.3).into();
                }
            }
        }
    }
}

// 数字表示システム - ゲーム状態と連携
pub fn number_display_system(
    game_numbers: Res<GameNumbers>,
    mut number_query: Query<(&NumberDisplay, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    // GameNumbersリソースが変更された場合のみ更新
    if game_numbers.is_changed() {
        for (number_display, children) in number_query.iter_mut() {
            // 対応する桁の値を取得
            let value = game_numbers.digits[number_display.index];
            
            // 子要素のテキストを更新
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    **text = value.to_string();
                }
            }
        }
    }
}