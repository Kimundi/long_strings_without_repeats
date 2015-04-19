#![feature(test)]

extern crate test;

extern crate long_strings_without_repeats as lswr;

// Common generic test
fn test_lswr<F>(lswr :F)
    where F: Fn(&mut [u8], u8) -> &mut [u8]
{
    // Test example in paper by mapping a = 0, b = 1, c = 2 ...
    let mut string = lswr::new_paper_example_string();
    assert_eq!(lswr(&mut string, 8),
               &mut [2, 1, 0, 1, 2, 1, 0, 2, 1, 2, 0, 1]);

    // WARNING! I'm uncertain whether anything below
    // is correct

    // Same string but in ascii/utf8 range
    let mut string: Vec<u8> = "cabageheadbag".into();
    assert_eq!(lswr(&mut string, 255),
               &mut [1, 2, 0, 2, 1, 0, 2, 1, 0, 1]);

    let mut string: Vec<u8> = "".into();
    assert_eq!(lswr(&mut string, 255),
               &mut []);
}

#[test]
fn test_lswr_rust_naive() {
    test_lswr(lswr::rust_naive::lswr)
}

#[test]
fn test_lswr_cpp_naive() {
    test_lswr(lswr::cpp_naive::lswr)
}

#[test]
fn test_lswr_rust_unsafe_index() {
    test_lswr(lswr::rust_unsafe_index::lswr)
}

#[test]
fn test_lswr_rust_unsafe_pointer() {
    test_lswr(lswr::rust_unsafe_pointer::lswr)
}

#[test]
fn test_comparison() {
    let example = lswr::new_paper_example_string();
    let repeats = (1024 * 1024) / example.len();
    let mut big_example = Vec::<u8>::with_capacity(example.len() * repeats);

    for _ in 0..repeats {
        big_example.extend(example.iter().cloned());
    }

    let mut string = big_example.clone();

    string.clone_from(&big_example);
    lswr::rust_naive::lswr(&mut string, 8);
    let a = string.clone();

    string.clone_from(&big_example);
    lswr::cpp_naive::lswr(&mut string, 8);
    let b = string.clone();

    string.clone_from(&big_example);
    lswr::rust_unsafe_index::lswr(&mut string, 8);
    let c = string.clone();

    string.clone_from(&big_example);
    lswr::rust_unsafe_pointer::lswr(&mut string, 8);
    let d = string.clone();

    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(c, d);
}
