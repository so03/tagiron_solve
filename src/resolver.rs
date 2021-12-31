use std::collections::HashSet;

use crate::{executor::*, models::{ALL_CARDS, Card}};
use itertools::Itertools;

pub struct Resolver {
    queries_bunch: Vec<Queries>,
    all_combs: VVCard,
}

impl Resolver {
    pub fn new(queries_bunch: Vec<Queries>) -> Self {
        let all_combs = ALL_CARDS.into_iter().combinations(4).collect();
        Resolver {
            queries_bunch,
            all_combs
        }
    }

    pub fn solve(&self) {
        let mut results = vec![];
        for queries in &self.queries_bunch {
            let executor = Executor::new();
            let result = executor.run(self.all_combs.clone(), queries);

            println!("{:?}", result);

            results.push(result);
        }

        let mut all_cards = self.all_combs.clone();

        let answers = HashSet::new();
        for r1 in results[0] {
            for r2 in results[1] {
                for r3 in results[2] {
                    for r4 in results[3] {
                       let all: HashSet<Card> = ALL_CARDS.iter().copied().collect(); 
                       let r1: HashSet<Card> = r1.iter().copied().collect();
                       let r2: HashSet<Card> = r2.iter().copied().collect();
                       let r3: HashSet<Card> = r3.iter().copied().collect();
                       let r4: HashSet<Card> = r4.iter().copied().collect();
                       let ans = all.difference(&r1);
                       let ans = all.difference(&r2);
                       let ans = all.difference(&r3);
                       let ans = all.difference(&r4);
                       answers.insert(ans);
                    }
                }
            }
        }
    }
}