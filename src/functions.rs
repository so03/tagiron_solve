use crate::executor::VVCard;
use crate::models::{Card, Color};

// 赤の数の合計は？
pub fn sum_red(v: VVCard, args: Vec<usize>) -> VVCard {
    v.into_iter()
        .filter(|v| v.iter().map(|c| c.number).sum::<u8>() == args[0] as u8)
        .collect()
}

// 赤の数字タイルは何枚ある？
pub fn how_many_red(v: VVCard, args: Vec<usize>) -> VVCard {
    v.into_iter()
        .filter(|v| v.iter().filter(|c| c.color == Color::Red).count() == args[0])
        .collect()
}

// 3はどこ？
pub fn where_is_three(v: VVCard, args: Vec<usize>) -> VVCard {
    v.into_iter()
        .filter(|v| args.iter().all(|i| v[*i].number == 3))
        .collect()
}

// 4はどこ？
pub fn where_is_four(v: VVCard, args: Vec<usize>) -> VVCard {
    v.into_iter()
        .filter(|v| args.iter().all(|i| v[*i].number == 4))
        .collect()
}
