use std::collections::HashSet;

use crate::{executor::*, cards::{ALL_CARDS, Card, all_cards_combs, Cards}};
use itertools::Itertools;

pub struct Resolver {
    queries_bunch: Vec<Queries>,
    all_combs: VVCard,
}

impl Resolver {
    pub fn new(queries_bunch: Vec<Queries>) -> Self {
        let all_combs = all_cards_combs();
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
                       let r1: HashSet<Card> = r1.collect();
                       let r2: HashSet<Card> = r2.collect();
                       let r3: HashSet<Card> = r3.collect();
                       let r4: HashSet<Card> = r4.collect();
                       let ans = all.difference(&r1);
                       let ans = all.difference(&r2);
                       let ans = all.difference(&r3);
                       let ans = all.difference(&r4);
                       let ans = Cards::new(ans.copied().collect());
                       answers.insert(ans);
                    }
                }
            }
        }

        if answers.len() == 1 {
            let ans = answers.iter().next().unwrap();
            println!("{:?}", ans);
        } else {
            println!("Can't understand yet.");
        }
    }
}