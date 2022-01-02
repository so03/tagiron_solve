use crate::{cards::Cards, queries::Queries};

pub struct Combs {
    v: Vec<Cards>
}

impl Combs {
    pub fn new(v: Vec<Cards>) -> Self {
        Self { v }
    }

    pub fn apply(self, qs: Queries) -> {
        for q in qs.iter() {
            self.v
        }
    }
}