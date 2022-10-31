use progress::*;

#[derive(Debug)]
pub struct Fib {
    a: u64,
    b: u64,
    len: usize,
}

impl Fib {
    pub fn new(len: usize) -> Self {
        Self { a: 0, b: 1, len }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let res = self.a;
        self.a = self.b;
        self.b = res + self.a;
        self.len -= 1;

        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl ExactSizeIterator for Fib {}

fn main() {
    let mut v = Vec::new();

    for n in Fib::new(27)
        .rate_limit(std::time::Duration::from_millis(150))
        .show_percent()
    {
        v.push(n);
    }

    println!("{v:?}");
}
