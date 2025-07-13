mod game;

use game::{Calculator, GameNumbers};

fn main() {
    println!("Make 10 Game - Starting...");

    // テスト用：数字生成の動作確認
    let numbers = GameNumbers::new();
    println!("Generated numbers: {:?}", numbers.digits);

    // 解答可能性をチェック
    let can_solve = Calculator::can_make_ten(&numbers);
    println!("Can make 10: {can_solve}");

    // いくつかの既知の組み合わせをテスト
    let test_cases = [[1, 2, 3, 4], [2, 5, 5, 0], [8, 2, 1, 5], [0, 0, 0, 0]];

    for digits in test_cases {
        let test_numbers = GameNumbers::from_digits(digits);
        let solvable = Calculator::can_make_ten(&test_numbers);
        println!("{digits:?} -> Can make 10: {solvable}");
    }
}
