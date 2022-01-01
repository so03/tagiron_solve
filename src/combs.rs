use crate::cards::Cards;

pub struct Combs {
    v: Vec<Cards>
}

impl Combs {
    pub fn new(v: Vec<Cards>) -> Self {
        Self { v }
    }
}