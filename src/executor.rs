use crate::functions::*;
use crate::models::{Card, Color, ALL_CARDS};
use itertools::Itertools;
use std::collections::HashMap;

pub type VVCard = Vec<Vec<Card>>;
type Question = &'static str;
type Args = Vec<usize>;
type Callback = fn(v: VVCard, args: Vec<usize>) -> VVCard;
type Queries = Vec<(Question, Args)>;

pub struct Executor {
    table: HashMap<&'static str, Callback>,
}

impl Executor {
    pub fn new() -> Self {
        let mut table: HashMap<&str, Callback> = HashMap::new();
        table.insert("where is three", where_is_three);
        table.insert("where is four", where_is_four);
        table.insert("sum red", sum_red);
        table.insert("how many red", how_many_red);

        Executor { table }
    }

    pub fn run(&self, mut cards: VVCard, queries: Queries) -> VVCard {
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
