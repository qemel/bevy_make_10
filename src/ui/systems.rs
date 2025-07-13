use super::components::*;
use crate::game::GameNumbers;
use bevy::prelude::*;

type ButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        Option<&'static NumberDisplay>,
        Option<&'static OperatorButton>,
        Option<&'static ResetButton>,
    ),
    (Changed<Interaction>, With<Button>),
>;

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
            // Title
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

            // Numbers display area
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    column_gap: Val::Px(20.0),
                    ..default()
                },))
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
                                NumberDisplay {
                                    value: digit_value as u32,
                                    index: i,
                                },
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
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    column_gap: Val::Px(15.0),
                    ..default()
                },))
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

            // 計算式と結果表示エリア
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    row_gap: Val::Px(10.0),
                    ..default()
                },))
                .with_children(|calc_parent| {
                    // 計算式表示
                    calc_parent.spawn((
                        Text::new("Expression: "),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ExpressionDisplay,
                    ));

                    // 計算結果表示
                    calc_parent.spawn((
                        Text::new("Result: "),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.2, 0.8, 0.2)),
                        ResultDisplay,
                    ));
                });

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
                        Text::new("Reset"),
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
    mut interaction_query: ButtonQuery,
    mut calc_state: ResMut<CalculationState>,
    game_numbers: Res<GameNumbers>,
) {
    for (interaction, mut color, number_display, operator_button, reset_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if let Some(number) = number_display {
                    // 数字ボタンが押された時の処理
                    let digit_value = game_numbers.digits[number.index];

                    // 新しい数字を式に追加
                    if calc_state.expression.is_empty() {
                        calc_state.expression = digit_value.to_string();
                    } else {
                        // 最後の文字が演算子の場合のみ数字を追加
                        if let Some(last_char) = calc_state.expression.chars().last() {
                            if "+-*/".contains(last_char) {
                                calc_state.expression.push_str(&format!(" {}", digit_value));

                                // 簡単な計算を実行（2項演算のみ）
                                if let Some(result) =
                                    evaluate_simple_expression(&calc_state.expression)
                                {
                                    calc_state.result = Some(result);
                                }
                            }
                        }
                    }

                    println!(
                        "Number button pressed: {} (index: {})",
                        number.value, number.index
                    );
                } else if let Some(operator) = operator_button {
                    // 演算子ボタンが押された時の処理
                    if !calc_state.expression.is_empty() {
                        // 最後の文字が数字の場合のみ演算子を追加
                        if let Some(last_char) = calc_state.expression.chars().last() {
                            if last_char.is_ascii_digit() {
                                calc_state
                                    .expression
                                    .push_str(&format!(" {} ", operator.operator));
                            }
                        }
                    }

                    println!("Operator button pressed: {}", operator.operator);
                } else if reset_button.is_some() {
                    // リセットボタンが押された時の処理
                    calc_state.expression.clear();
                    calc_state.result = None;
                    calc_state.selected_numbers.clear();
                    calc_state.operators.clear();

                    println!("Reset button pressed");
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

// 計算表示システム - 計算式と結果の更新
pub fn calculation_display_system(
    calc_state: Res<CalculationState>,
    mut expr_query: Query<&mut Text, (With<ExpressionDisplay>, Without<ResultDisplay>)>,
    mut result_query: Query<&mut Text, (With<ResultDisplay>, Without<ExpressionDisplay>)>,
) {
    // CalculationStateが変更された場合のみ更新
    if calc_state.is_changed() {
        // 計算式表示の更新
        if let Ok(mut expr_text) = expr_query.single_mut() {
            if calc_state.expression.is_empty() {
                **expr_text = "Expression: ".to_string();
            } else {
                **expr_text = format!("Expression: {}", calc_state.expression);
            }
        }

        // 計算結果表示の更新
        if let Ok(mut result_text) = result_query.single_mut() {
            match calc_state.result {
                Some(result) => {
                    **result_text = format!("Result: {}", result);
                }
                None => {
                    **result_text = "Result: ".to_string();
                }
            }
        }
    }
}

// 簡単な計算式評価関数（2項演算のみ）
fn evaluate_simple_expression(expression: &str) -> Option<f64> {
    let parts: Vec<&str> = expression.split_whitespace().collect();

    // 最低でも3つの部分（数字 演算子 数字）が必要
    if parts.len() >= 3 {
        let left = parts[0].parse::<f64>().ok()?;
        let operator = parts[1];
        let right = parts[2].parse::<f64>().ok()?;

        match operator {
            "+" => Some(left + right),
            "-" => Some(left - right),
            "*" => Some(left * right),
            "/" => {
                if right != 0.0 {
                    Some(left / right)
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}
