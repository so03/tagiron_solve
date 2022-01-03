use itertools::Itertools;

use crate::cards::{Cards, Color, Card};


// use crate::executor::VVCard;
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
// // 赤の数の合計は？
// pub fn sum_red(v: VVCard, args: Vec<usize>) -> VVCard {
//     v.into_iter()
//         .filter(|v| v.map(|c| c.number).sum::<u8>() == args[0] as u8)
//         .collect()
// }

// pub fn sum_red_bool(v: Vec<Card>, args: Vec<usize>) -> bool {
//     let sum = v
//         .into_iter()
//         .filter(|c| c.is_red())
//         .map(|c| c.number)
//         .sum::<u8>();
//     sum == args[0] as u8
// }

// // 赤の数字タイルは何枚ある？
// pub fn how_many_red(v: VVCard, args: Vec<usize>) -> VVCard {
//     v.into_iter()
//         .filter(|v| v.filter(|c| c.color == Color::Red).count() == args[0])
//         .collect()
// }

// // 3はどこ？
// pub fn where_is_three(v: VVCard, args: Vec<usize>) -> VVCard {
//     v.into_iter()
//         .filter(|v| {
//             let mut flag = true;
//             for (i, c) in v.enumerate() {
//                 if args.iter().any(|j| *j == i) && c.number != 3 {
//                     flag = false;
//                 }
//             }
//             flag
//         })
//         .collect()
// }

// // 4はどこ？
// pub fn where_is_four(v: VVCard, args: Vec<usize>) -> VVCard {
//     v.into_iter()
//         .filter(|v| {
//             let mut flag = true;
//             for (i, c) in v.enumerate() {
//                 if args.iter().any(|j| *j == i) && c.number != 4 {
//                     flag = false;
//                 }
//             }
//             flag
//         })
//         .collect()
// }
