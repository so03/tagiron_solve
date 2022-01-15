use itertools::Itertools;

use crate::cards::{Card, Cards, Color};

// use crate::executor::&Cards;
// use crate::cards::{Card, Color};

// ForDebug
// args: [1, 2, 3, 4, 1, 1, 2, 2]. 0..4 is number, 4..8 is color.
pub fn what_is_your_answer(cs: &Cards, args: Vec<usize>) -> bool {
    let colors = args[4..8].iter().map(|i| match *i {
        1 => Color::Red,
        2 => Color::Blue,
        _ => Color::Yellow,
    });
    let cards = args[0..4]
        .iter()
        .zip(colors)
        .map(|(&number, color)| Card { number, color })
        .collect_vec();
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

// 0はどこ？
pub fn where_is_zero(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(0);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
}

// 1はどこ？
pub fn where_is_one(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(1);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
}

// 2はどこ？
pub fn where_is_two(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(2);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
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

// 5はどこ？
pub fn where_is_five(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(5);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
}

// 6はどこ？
pub fn where_is_six(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(6);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
}

// 7はどこ？
pub fn where_is_seven(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(7);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
}

// 8はどこ？
pub fn where_is_eight(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(8);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
}

// 9はどこ？
pub fn where_is_nine(v: &Cards, args: Vec<usize>) -> bool {
    let mut a = v.positions_of(9);
    a.sort_unstable();
    let mut b = args;
    b.sort_unstable();
    a == b
}

// 同じ色が隣り合っている数字タイルはどこ？
pub fn where_is_same_color(v: &Cards, args: Vec<usize>) -> bool {
    assert!(args.len() % 2 == 0);
    args.chunks(2).all(|args| v.is_same_color(args[0], args[1]))
}

//  [共有情報カード]数字タイル全ての数の合計は？
pub fn sum_all(v: &Cards, args: Vec<usize>) -> bool {
    v.sum() == args[0]
}

//  偶数は何枚ある？（0も含む）
pub fn even_count(v: &Cards, args: Vec<usize>) -> bool {
    v.evens().count() == args[0]
}

//  奇数は何枚ある？
pub fn odd_count(v: &Cards, args: Vec<usize>) -> bool {
    v.odds().count() == args[0]
}

//  大きい方から3枚の数の合計は？
pub fn big_three_sum(v: &Cards, args: Vec<usize>) -> bool {
    v.big_three_sum() == args[0]
}

//  小さい方から3枚の数の合計は？
pub fn small_three_sum(v: &Cards, args: Vec<usize>) -> bool {
    v.small_three_sum() == args[0]
}

//  数が連続している数字タイルはどこ？
pub fn where_is_continuous(v: &Cards, args: Vec<usize>) -> bool {
    let mut f = true;
    for (i, j) in v.positions_of_continuous().iter().zip(&args) {
        if i != j {
            f = false;
        }
    }
    f
}
//  同じ数字タイルのペアは何組ある？
//  [共有情報カード]数字タイルの最大の数から、最小の数を引いた数は？

//  [共有情報カード]中央の数字タイルは5以上？4以下？
//  中央の3枚の数の合計は？
