#![feature(test)]

extern crate test;
use self::test::{Bencher, black_box};

extern crate long_strings_without_repeats as lswr;
use lswr::{rust_iter, rust_index, cpp_index};

#[bench]
fn bench_rust_iter_short(bencher: &mut Bencher) {
    let example = lswr::new_paper_example_string();
    let mut string = example.clone();

    bencher.iter(|| {
        string.clone_from(&example);
        black_box(rust_iter::lswr(&mut string, 8));
    });
}

#[bench]
fn bench_rust_index_short(bencher: &mut Bencher) {
    let example = lswr::new_paper_example_string();
    let mut string = example.clone();

    bencher.iter(|| {
        string.clone_from(&example);
        black_box(rust_index::lswr(&mut string, 8));
    });
}

#[bench]
fn bench_cpp_index_short(bencher: &mut Bencher) {
    let example = lswr::new_paper_example_string();
    let mut string = example.clone();

    bencher.iter(|| {
        string.clone_from(&example);
        black_box(cpp_index::lswr(&mut string, 8));
    });
}
