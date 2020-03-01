#![feature(test)]

extern crate test;

pub fn find_nth_odd(n: u64) -> u64 {
    let mut i = 0;
    let mut odd_count = 0;

    while odd_count != n {
        i += 1;
        if is_odd(i) {
            odd_count += 1;
        }
    }

    i
}

fn is_odd(n: u64) -> bool {
    n % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_no_indirection(b: &mut Bencher) {
        b.iter(|| find_nth_odd(test::black_box(1_000_000_000_000_000)))
    }
}
