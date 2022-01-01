pub mod executor;
pub mod functions;
pub mod models;
pub mod resolver;

use executor::*;
use itertools::Itertools;
use models::Card;

use crate::models::ALL_CARDS;

use std::collections::HashSet;

fn main() {
    let queries = todo!();

    let c = todo!(); // cards, except mine

    // all combinations
    let comb = todo!();

    // expectations of answers
    let mut ap = vec![];

    let comb1 = comb.apply(queries[0]);
    for cp1 in comb1 {
        let c = c.difference(cp1);
        let comb2 = c.combinations();
        let comb2 = comb2.apply();
        for cp2 in comb2 {
            let c = c.difference(cp2);
            let comb3 = c.combinations();
            let comb3 = comb3.apply();
            for cp3 in comb3 {
                let ans = c.difference(cp3);
                if ans.len() == 4 {
                    ap.push(ans);
                }
                // let comb4 = c.combinations();
            }
        }
    }

    ap.uniq();

    if ap.len() == 1 {
        println!("Answer is decided.");
        println!("{:?}", ap);
    } else {
        println!("Answer is not decided.");
        println!("{:?}", ap);
    }
}

#[test]
fn count_of_all_combs() {
    let all_combs = ALL_CARDS.iter().combinations(4);
    assert_eq!(4845, all_combs.count());
}
