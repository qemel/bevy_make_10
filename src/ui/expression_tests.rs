#[cfg(test)]
mod tests {
    use super::super::systems::evaluate_expression;

    // 基本的な2項演算のテスト
    #[test]
    fn test_basic_addition() {
        assert_eq!(evaluate_expression("2 + 3"), Some(5.0));
        assert_eq!(evaluate_expression("1 + 9"), Some(10.0));
    }

    #[test]
    fn test_basic_subtraction() {
        assert_eq!(evaluate_expression("5 - 2"), Some(3.0));
        assert_eq!(evaluate_expression("9 - 4"), Some(5.0));
    }

    #[test]
    fn test_basic_multiplication() {
        assert_eq!(evaluate_expression("3 * 4"), Some(12.0));
        assert_eq!(evaluate_expression("2 * 5"), Some(10.0));
    }

    #[test]
    fn test_basic_division() {
        assert_eq!(evaluate_expression("8 / 2"), Some(4.0));
        assert_eq!(evaluate_expression("9 / 3"), Some(3.0));
    }

    // 演算子優先度のテスト
    #[test]
    fn test_operator_precedence_multiply_first() {
        assert_eq!(evaluate_expression("2 + 3 * 4"), Some(14.0)); // 2 + (3 * 4) = 2 + 12 = 14
        assert_eq!(evaluate_expression("1 + 2 * 3"), Some(7.0)); // 1 + (2 * 3) = 1 + 6 = 7
    }

    #[test]
    fn test_operator_precedence_divide_first() {
        assert_eq!(evaluate_expression("8 + 6 / 2"), Some(11.0)); // 8 + (6 / 2) = 8 + 3 = 11
        assert_eq!(evaluate_expression("1 + 8 / 4"), Some(3.0)); // 1 + (8 / 4) = 1 + 2 = 3
    }

    #[test]
    fn test_operator_precedence_mixed() {
        assert_eq!(evaluate_expression("2 + 3 * 4 - 1"), Some(13.0)); // 2 + (3 * 4) - 1 = 2 + 12 - 1 = 13
        assert_eq!(evaluate_expression("9 - 6 / 2 + 1"), Some(7.0)); // 9 - (6 / 2) + 1 = 9 - 3 + 1 = 7
    }

    // 4項演算のテスト
    #[test]
    fn test_four_operand_expressions() {
        assert_eq!(evaluate_expression("1 + 2 + 3 + 4"), Some(10.0));
        assert_eq!(evaluate_expression("2 * 3 + 4 / 2"), Some(8.0)); // (2 * 3) + (4 / 2) = 6 + 2 = 8
        assert_eq!(evaluate_expression("9 - 3 + 2 * 2"), Some(10.0)); // 9 - 3 + (2 * 2) = 9 - 3 + 4 = 10
    }

    // ゼロ除算のテスト
    #[test]
    fn test_division_by_zero() {
        assert_eq!(evaluate_expression("5 / 0"), None);
        assert_eq!(evaluate_expression("1 + 8 / 0"), None);
    }

    // 無効な入力のテスト
    #[test]
    fn test_invalid_short_expressions() {
        assert_eq!(evaluate_expression("1"), None);
        assert_eq!(evaluate_expression("1 +"), None);
        assert_eq!(evaluate_expression("+ 2"), None);
        assert_eq!(evaluate_expression(""), None);
    }

    #[test]
    fn test_invalid_two_digit_numbers() {
        assert_eq!(evaluate_expression("10 + 5"), None); // 2桁数字は無効
        assert_eq!(evaluate_expression("1 + 15"), None); // 2桁数字は無効
        assert_eq!(evaluate_expression("12 * 3"), None); // 2桁数字は無効
    }

    #[test]
    fn test_invalid_zero_numbers() {
        assert_eq!(evaluate_expression("0 + 5"), None); // 0は無効
        assert_eq!(evaluate_expression("1 + 0"), None); // 0は無効
    }

    #[test]
    fn test_invalid_decimal_numbers() {
        assert_eq!(evaluate_expression("1.5 + 2"), None); // 小数は無効
        assert_eq!(evaluate_expression("1 + 2.3"), None); // 小数は無効
    }

    #[test]
    fn test_invalid_operators() {
        assert_eq!(evaluate_expression("1 & 2"), None); // 無効な演算子
        assert_eq!(evaluate_expression("1 ^ 2"), None); // 無効な演算子
        assert_eq!(evaluate_expression("1 % 2"), None); // 無効な演算子
    }

    #[test]
    fn test_invalid_consecutive_operators() {
        assert_eq!(evaluate_expression("1 + + 2"), None); // 連続する演算子
        assert_eq!(evaluate_expression("1 * - 2"), None); // 連続する演算子
        assert_eq!(evaluate_expression("1 + * 2"), None); // 連続する演算子
    }

    #[test]
    fn test_invalid_even_number_of_parts() {
        assert_eq!(evaluate_expression("1 + 2 +"), None); // 演算子で終わる
        assert_eq!(evaluate_expression("+ 1 + 2"), None); // 演算子で始まる
    }

    #[test]
    fn test_invalid_alphabetic_input() {
        assert_eq!(evaluate_expression("a + b"), None); // 文字は無効
        assert_eq!(evaluate_expression("1 + x"), None); // 文字は無効
    }

    // Make 10 の典型的なケースのテスト
    #[test]
    fn test_make_10_cases() {
        assert_eq!(evaluate_expression("1 + 2 + 3 + 4"), Some(10.0));
        assert_eq!(evaluate_expression("2 * 5"), Some(10.0));
        assert_eq!(evaluate_expression("3 + 4 + 2 + 1"), Some(10.0));
        assert_eq!(evaluate_expression("6 + 8 / 2"), Some(10.0)); // 6 + (8 / 2) = 6 + 4 = 10
        assert_eq!(evaluate_expression("4 * 3 - 2"), Some(10.0)); // (4 * 3) - 2 = 12 - 2 = 10
    }

    // エッジケースのテスト
    #[test]
    fn test_edge_cases() {
        assert_eq!(evaluate_expression("9 * 1"), Some(9.0));
        assert_eq!(evaluate_expression("1 * 9"), Some(9.0));
        assert_eq!(evaluate_expression("9 / 9"), Some(1.0));
        assert_eq!(evaluate_expression("9 - 9"), Some(0.0));
        assert_eq!(evaluate_expression("1 + 1 + 1 + 1"), Some(4.0));
    }

    // 空白の処理テスト
    #[test]
    fn test_whitespace_handling() {
        assert_eq!(evaluate_expression("  2  +  3  "), Some(5.0));
        assert_eq!(evaluate_expression("2+3"), None); // 空白なしは無効（現在の実装では）
    }
}
