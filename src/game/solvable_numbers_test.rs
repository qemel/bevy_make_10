#[cfg(test)]
mod tests {
    use crate::game::{GameNumbers, Calculator};

    #[test]
    fn test_all_generated_numbers_are_solvable() {
        // 複数回生成して、すべてが解けることを確認
        for i in 0..100 {
            let numbers = GameNumbers::new();
            
            assert!(
                Calculator::can_make_ten(&numbers),
                "生成された数字 {:?} (試行 {}) で10を作ることができません",
                numbers.digits,
                i + 1
            );
        }
    }

    #[test]
    fn test_from_seed_preserves_determinism() {
        // from_seedメソッドの決定性を確認（解けることは保証しない）
        for seed in [12345, 67890, 99999, 11111, 55555] {
            let numbers1 = GameNumbers::from_seed(seed);
            let numbers2 = GameNumbers::from_seed(seed);
            
            assert_eq!(
                numbers1.digits, numbers2.digits,
                "シード {} から生成された数字が一貫していません",
                seed
            );
        }
    }

    #[test]
    fn test_generated_numbers_have_valid_digits() {
        // 生成された数字が1-9の範囲内であることを確認（Make10ゲーム用）
        for _ in 0..50 {
            let numbers = GameNumbers::new();
            
            for &digit in &numbers.digits {
                assert!(
                    digit >= 1 && digit <= 9,
                    "生成された数字 {} が有効範囲（1-9）外です。数字セット: {:?}",
                    digit,
                    numbers.digits
                );
            }
        }
    }

    #[test]
    fn test_known_solvable_combinations() {
        // 既知の解ける組み合わせをテスト
        let solvable_sets = [
            [1, 2, 3, 4], // 1 + 2 + 3 + 4 = 10
            [2, 5, 1, 2], // 2 * 5 + 1 - 2 = 9（実際は別の組み合わせで10）
            [8, 2, 1, 5], // 8 / 2 + 1 + 5 = 10
            [3, 3, 3, 1], // 3 * 3 + 3 - 1 = 11（実際は別の組み合わせで10）
            [6, 4, 2, 2], // 6 + 4 = 10
        ];

        for digits in solvable_sets {
            let numbers = GameNumbers::from_digits(digits);
            assert!(
                Calculator::can_make_ten(&numbers),
                "既知の解ける組み合わせ {:?} で10を作ることができません",
                digits
            );
        }
    }

    #[test]
    fn test_unsolvable_combinations_fail() {
        // 解けない組み合わせが正しく失敗することを確認
        let unsolvable_sets = [
            [0, 0, 0, 0], // すべて0
            [1, 1, 1, 1], // すべて1（1+1+1+1=4, 1*1*1*1=1など）
        ];

        for digits in unsolvable_sets {
            let numbers = GameNumbers::from_digits(digits);
            assert!(
                !Calculator::can_make_ten(&numbers),
                "解けないはずの組み合わせ {:?} で10が作れてしまいます",
                digits
            );
        }
    }
}