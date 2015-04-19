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
fn test_lswr_rust_unsafe() {
    test_lswr(lswr::rust_unsafe::lswr)
}
