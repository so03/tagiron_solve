use itertools::Itertools;
use std::collections::HashMap;

type VVCard = Vec<Vec<Card>>;
type Question = &'static str;
type Args = Vec<usize>;
type Callback = fn(v: VVCard, args: Vec<usize>) -> VVCard;
type Queries = Vec<(Question, Args)>;

fn main() {
    let all_combs = ALL_CARDS.into_iter().combinations(4).collect();

    let executor = Executor::new();

    let queries = vec![("sum red", vec![6]), ("how many red", vec![4])];

    let result = executor.run(all_combs, queries);
    println!("{:?}", result);
}

struct Executor {
    table: HashMap<&'static str, Callback>,
}

impl Executor {
    fn new() -> Self {
        let mut table: HashMap<&str, Callback> = HashMap::new();
        table.insert("where is three", where_is_three);
        table.insert("where is four", where_is_four);
        table.insert("sum red", sum_red);
        table.insert("how many red", how_many_red);

        Executor { table }
    }

    fn run(&self, mut cards: VVCard, queries: Queries) -> VVCard {
        for (q, args) in queries {
            cards = self.table[q](cards, args)
        }
        cards
    }
}

#[test]
fn if_all_red_and_minimum_sum() {
    let all_combs = ALL_CARDS.into_iter().combinations(4).collect();

    let executor = Executor::new();

    let queries = vec![("sum red", vec![6]), ("how many red", vec![4])];

    let result = executor.run(all_combs, queries);

    assert_eq!(
        result,
        vec![vec![
            Card {
                number: 0,
                color: Color::Red
            },
            Card {
                number: 1,
                color: Color::Red
            },
            Card {
                number: 2,
                color: Color::Red
            },
            Card {
                number: 3,
                color: Color::Red
            }
        ]]
    );
}

// 赤の数の合計は？
fn sum_red(v: VVCard, args: Vec<usize>) -> VVCard {
    v.into_iter()
        .filter(|v| v.iter().map(|c| c.number).sum::<u8>() == args[0] as u8)
        .collect()
}

// 赤の数字タイルは何枚ある？
fn how_many_red(v: VVCard, args: Vec<usize>) -> VVCard {
    v.into_iter()
        .filter(|v| v.iter().filter(|c| c.color == Color::Red).count() == args[0])
        .collect()
}

// 3はどこ？
fn where_is_three(v: VVCard, args: Vec<usize>) -> VVCard {
    v.into_iter()
        .filter(|v| args.iter().all(|i| v[*i].number == 3))
        .collect()
}

// 4はどこ？
fn where_is_four(v: VVCard, args: Vec<usize>) -> VVCard {
    v.into_iter()
        .filter(|v| args.iter().all(|i| v[*i].number == 4))
        .collect()
}

#[test]
fn count_of_all_combs() {
    let all_combs = ALL_CARDS.iter().combinations(4);
    assert_eq!(4845, all_combs.count());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    number: u8,
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red,
    Blue,
    Yellow,
}

pub const ALL_CARDS: [Card; 20] = [
    Card {
        number: 0,
        color: Color::Red,
    },
    Card {
        number: 1,
        color: Color::Red,
    },
    Card {
        number: 2,
        color: Color::Red,
    },
    Card {
        number: 3,
        color: Color::Red,
    },
    Card {
        number: 4,
        color: Color::Red,
    },
    Card {
        number: 5,
        color: Color::Yellow,
    },
    Card {
        number: 6,
        color: Color::Red,
    },
    Card {
        number: 7,
        color: Color::Red,
    },
    Card {
        number: 8,
        color: Color::Red,
    },
    Card {
        number: 9,
        color: Color::Red,
    },
    Card {
        number: 0,
        color: Color::Blue,
    },
    Card {
        number: 1,
        color: Color::Blue,
    },
    Card {
        number: 2,
        color: Color::Blue,
    },
    Card {
        number: 3,
        color: Color::Blue,
    },
    Card {
        number: 4,
        color: Color::Blue,
    },
    Card {
        number: 5,
        color: Color::Yellow,
    },
    Card {
        number: 6,
        color: Color::Blue,
    },
    Card {
        number: 7,
        color: Color::Blue,
    },
    Card {
        number: 8,
        color: Color::Blue,
    },
    Card {
        number: 9,
        color: Color::Blue,
    },
];
