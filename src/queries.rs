use crate::cards::Card;

pub struct Queries<T>
{
    v: Vec<T>,
}

impl<T> Queries<T>
where T: Query,
T::T: Iterator<Item = Card>
{
    pub fn new(v: Vec<T>) -> Self {
        Queries { v }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.v.iter()
    }
}

trait Query
{
    type T;
    fn call(cs: Self::T, args: &[usize]) -> Self::T;
}