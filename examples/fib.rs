use num::{BigInt, One, Zero};
use progress::*;
use std::mem;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Fib<T> {
    a: T,
    b: T,
    len: usize,
}

impl<T: Zero + One> Fib<T> {
    pub fn new(len: usize) -> Self {
        Self {
            a: T::zero(),
            b: T::one(),
            len,
        }
    }
}

pub trait AddAssignRef: Sized + AddAssign + for<'r> AddAssign<&'r Self> {}
impl<T> AddAssignRef for T where T: AddAssign + for<'r> AddAssign<&'r T> {}

impl<T: Clone + AddAssignRef> Iterator for Fib<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;

        let res = self.a.clone();
        mem::swap(&mut self.a, &mut self.b);
        self.b += &self.a;

        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T: Clone + AddAssignRef> ExactSizeIterator for Fib<T> {}

fn main() {
    let mut v = Vec::<BigInt>::new();

    for n in Fib::new(102043).show_percent() {
        v.push(n);
    }

    println!(
        "The 102043th member of the fibonaci sequence is {} digits long",
        v.last().unwrap().to_string().len()
    );
}
