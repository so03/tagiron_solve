pub mod cards;
pub mod combs;
pub mod executor;
pub mod functions;
pub mod resolver;

use cards::Cards;

fn main() {
    let queries = todo!();

    let c = Cards::all();

    // mine
    let m = Cards::new(vec![]);

    // except mine
    let c = c.difference(&m);

    // all combinations without mine
    let combs = c.combs();

    // expectations of answers
    let mut ap = vec![];

    let combs1 = combs.apply(queries[0]);
    for cp1 in combs1 {
        let c = c.difference(cp1);
        let combs2 = c.combs();
        let combs2 = combs2.apply(queries[1]);
        for cp2 in combs2 {
            let c = c.difference(cp2);
            let combs3 = c.combs();
            let combs3 = combs3.apply(queries[2]);
            for cp3 in combs3 {
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
