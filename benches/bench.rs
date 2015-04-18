#![feature(test)]

extern crate test;
use self::test::{Bencher, black_box};

extern crate long_strings_without_repeats as lswr;
use lswr::{rust_naive, cpp_naive, rust_iter};

// Common generic bencher for a short string
fn bench_lswr_short<F>(bencher: &mut Bencher, lswr: F)
    where F: Fn(&mut [u8], u8) -> &mut [u8]
{
    let example = lswr::new_paper_example_string();
    let mut string = example.clone();

    bencher.iter(|| {
        string.clone_from(&example);
        black_box(lswr(&mut string, 8));
    });
}

#[bench]
fn bench_rust_naive_short(bencher: &mut Bencher) {
    bench_lswr_short(bencher, rust_naive::lswr);
}

#[bench]
fn bench_cpp_naive_short(bencher: &mut Bencher) {
    bench_lswr_short(bencher, cpp_naive::lswr);
}

#[bench]
fn bench_rust_iter_short(bencher: &mut Bencher) {
    bench_lswr_short(bencher, rust_iter::lswr);
}

// Common generic bencher for a big (1mb) string
fn bench_lswr_big<F>(bencher: &mut Bencher, lswr: F)
    where F: Fn(&mut [u8], u8) -> &mut [u8]
{
    let example = lswr::new_paper_example_string();
    let repeats = (1024 * 1024) / example.len();
    let mut big_example = Vec::<u8>::with_capacity(example.len() * repeats);

    for _ in 0..repeats {
        big_example.extend(example.iter().cloned());
    }

    let mut string = big_example.clone();

    bencher.iter(|| {
        string.clone_from(&big_example);
        black_box(lswr(&mut string, 8));
    });
}

#[bench]
fn bench_rust_naive_big(bencher: &mut Bencher) {
    bench_lswr_big(bencher, rust_naive::lswr);
}

#[bench]
fn bench_cpp_naive_big(bencher: &mut Bencher) {
    bench_lswr_big(bencher, cpp_naive::lswr);
}

#[bench]
fn bench_rust_iter_big(bencher: &mut Bencher) {
    bench_lswr_big(bencher, rust_iter::lswr);
}
