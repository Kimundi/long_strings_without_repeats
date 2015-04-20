#![feature(libc)]

pub mod rust_naive;
pub mod cpp_naive;

pub mod rust_unsafe;
pub mod rust_naive_parallel;

/// Create example in paper by mapping a = 0, b = 1, c = 2 ...
pub fn new_paper_example_string() -> Vec<u8> {
    "cabageheadbag".chars().map(|c| (c as u8) - b'a').collect()
}

/// returns bit at index i
fn bit(i: u8, byte: u8) -> u8 {
    (byte >> i) & 1
}

/// Returns rounded up binary logarithm of n, aka `ceil(log2(n))`.
/// This is only defined for n >= 1.
fn log2_ceil(n: u8) -> u8 {
    // see http://en.wikipedia.org/wiki/Binary_logarithm#Integer_rounding
    (8 - (n-1).leading_zeros()) as u8
}

/// Return the index of the least significant bit that differs
fn lsb_differ_index(a: u8, b: u8) -> u8 {
    // xor => bits with differences are 1
    // trailing_zero => position of first difference
    (a ^ b).trailing_zeros() as u8
}

#[test]
fn test_bit() {
    assert_eq!(bit(0, 0xff), 1);
    assert_eq!(bit(7, 0xff), 1);
    assert_eq!(bit(3, 0xff), 1);

    assert_eq!(bit(0, 0), 0);
    assert_eq!(bit(7, 0), 0);
    assert_eq!(bit(3, 0), 0);

    assert_eq!(bit(0, 0b0000_0001), 1);
    assert_eq!(bit(1, 0b0000_0010), 1);
    assert_eq!(bit(2, 0b0000_0100), 1);
    assert_eq!(bit(3, 0b0000_1000), 1);
    assert_eq!(bit(4, 0b0001_0000), 1);
    assert_eq!(bit(5, 0b0010_0000), 1);
    assert_eq!(bit(6, 0b0100_0000), 1);
    assert_eq!(bit(7, 0b1000_0000), 1);
}

#[test]
fn test_log2_ceil() {
    assert_eq!(log2_ceil(255), 8);
    assert_eq!(log2_ceil(17), 5);
    assert_eq!(log2_ceil(16), 4);
    assert_eq!(log2_ceil(15), 4);
    assert_eq!(log2_ceil(9), 4);
    assert_eq!(log2_ceil(8), 3);
    assert_eq!(log2_ceil(6), 3);
}

#[test]
fn test_lsb_differ_index() {
    assert_eq!(lsb_differ_index(0, 0), 8);
    assert_eq!(lsb_differ_index(0b0, 0b1), 0);
    assert_eq!(lsb_differ_index(0b00, 0b10), 1);
    assert_eq!(lsb_differ_index(0b100, 0b010), 1);
    assert_eq!(lsb_differ_index(0b1001, 0b0101), 2);
}
