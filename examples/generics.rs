use std::fmt::Display;

fn main() {}

struct Hoge<T, U>
    where T: MyTrait<U>,
    U: Display
{

}

trait MyTrait<T> {}