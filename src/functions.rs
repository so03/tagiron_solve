// use crate::executor::VVCard;
// use crate::cards::{Card, Color};

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
