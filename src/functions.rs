use itertools::Itertools;

use crate::cards::{Cards, Color, Card};


// use crate::executor::&Cards;
// use crate::cards::{Card, Color};

// ForDebug
// args: [1, 2, 3, 4, 1, 1, 2, 2]. 0..4 is number, 4..8 is color.
pub fn what_is_your_answer(cs: &Cards, args:Vec<usize>) -> bool {
    let colors = args[4..8].iter().map(|i| match *i {
        1 => Color::Red,
        2 => Color::Blue,
        _ => Color::Yellow
    });
    let cards = args[0..4].iter().zip(colors).map(|(&number, color)| {
        Card { number, color }
    }).collect_vec();
    let cards = Cards::new(cards);

    cs == &cards
}
// 赤の数の合計は？
pub fn sum_red(v: &Cards, args: Vec<usize>) -> bool {
    v.red_sums() == args[0]
}

// 青の数の合計は？
pub fn sum_blue(v: &Cards, args: Vec<usize>) -> bool {
    v.blue_sums() == args[0]
}

// 赤の数字タイルは何枚ある？
pub fn how_many_red(v: &Cards, args: Vec<usize>) -> bool {
    v.red_counts() == args[0]
}

// 3はどこ？
pub fn where_is_three(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(3);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
}

// 4はどこ？
pub fn where_is_four(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(4);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
}