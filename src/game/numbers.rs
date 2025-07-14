//! 数字生成とランダム4桁の管理

use bevy::prelude::*;

/// 4つのランダム数字を表す構造体
#[derive(Debug, Clone, PartialEq, Resource)]
pub struct GameNumbers {
    pub digits: [u8; 4],
}

impl GameNumbers {
    /// 新しいランダムな4桁を生成（必ず解ける組み合わせ）
    pub fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        use crate::game::Calculator;

        // 解ける組み合わせが見つかるまで生成を続ける
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        loop {
            let candidate = Self::from_seed_with_valid_range(seed);
            if Calculator::can_make_ten(&candidate) {
                return candidate;
            }
            // 次のシードを試す
            seed = seed.wrapping_add(1);
        }
    }

    /// シード値から決定的に4桁を生成（テスト用）
    pub fn from_seed(mut seed: u64) -> Self {
        let mut digits = [0u8; 4];

        for i in 0..4 {
            // 線形合同法による疑似乱数
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            digits[i] = ((seed / 65536) % 10) as u8;
        }

        Self { digits }
    }

    /// シード値から有効範囲（1-9）の4桁を生成
    fn from_seed_with_valid_range(mut seed: u64) -> Self {
        let mut digits = [0u8; 4];

        for i in 0..4 {
            // 線形合同法による疑似乱数
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            // 1-9の範囲で生成（0は除外）
            digits[i] = ((seed / 65536) % 9) as u8 + 1;
        }

        Self { digits }
    }

    /// 指定した数字から作成（テスト用）
    pub fn from_digits(digits: [u8; 4]) -> Self {
        Self { digits }
    }

    /// 各桁が0-9の範囲内かチェック
    pub fn is_valid(&self) -> bool {
        self.digits.iter().all(|&d| d <= 9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_generates_four_digits() {
        // テスト1: 新しい数字生成で4桁が生成されることを確認
        let numbers = GameNumbers::new();
        assert_eq!(numbers.digits.len(), 4);
    }

    #[test]
    fn test_digits_are_in_valid_range() {
        // テスト2: 生成された数字が0-9の範囲内であることを確認
        let numbers = GameNumbers::new();
        assert!(numbers.is_valid());

        for &digit in &numbers.digits {
            assert!(digit <= 9, "Digit {digit} is out of range");
        }
    }

    #[test]
    fn test_from_seed_is_deterministic() {
        // テスト3: 同じシードから同じ数字が生成されることを確認
        let seed = 12345;
        let numbers1 = GameNumbers::from_seed(seed);
        let numbers2 = GameNumbers::from_seed(seed);

        assert_eq!(numbers1, numbers2);
    }

    #[test]
    fn test_from_digits_creates_correct_numbers() {
        // テスト4: 指定した数字から正しく作成されることを確認
        let digits = [1, 2, 3, 4];
        let numbers = GameNumbers::from_digits(digits);

        assert_eq!(numbers.digits, digits);
    }

    #[test]
    fn test_is_valid_with_valid_digits() {
        // テスト5: 有効な数字の検証
        let numbers = GameNumbers::from_digits([0, 5, 9, 3]);
        assert!(numbers.is_valid());
    }

    #[test]
    fn test_is_valid_with_invalid_digits() {
        // テスト6: 無効な数字の検証（このテストは現在の実装では通らない）
        // 注意: from_digitsは現在どんな値でも受け入れるため、
        // より厳密な検証が必要な場合は実装を改善する
        let numbers = GameNumbers {
            digits: [0, 5, 10, 3],
        };
        assert!(!numbers.is_valid());
    }
}
