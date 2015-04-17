/// return the index of the least significant bit that differs
fn lsb_differ_index(a: u8, b: u8) -> u8 {
    // xor => bits mit underschieden sind 1
    // trailing_zero => position des ersten unterschieds
    (a ^ b).trailing_zeros() as u8
}

// returns bit at index i
fn bit(i: u8, byte: u8) -> u8 {
    (byte >> i) & 1
}

// returns rounded up binary logarithm of n, that is it returns `ceil(log2(n))`.
// only defined for n >= 1.
fn log2_ceil(n: u8) -> u8 {
    // see http://en.wikipedia.org/wiki/Binary_logarithm#Integer_rounding
    (8 - (n-1).leading_zeros()) as u8
}

// alphabet reduction label calculation
fn label(a_i_minus_one: u8, a_i: u8) -> u8 {
    let l = lsb_differ_index(a_i_minus_one, a_i);
    2 * l + bit(l, a_i)
}

/// longest strings without repreats algorithm
pub fn lswr(mut a: Vec<u8>, alpha_size: u8) -> Vec<u8> {
    {
        let mut new_a = phase1(&mut *a, alpha_size);
        phase2(new_a);
    }

    a
}

fn phase1(mut a: &mut [u8], mut alpha_size: u8) -> &mut [u8] {
    let mut len = a.len();

    // endless loop
    loop {

        // println!("{:?} alpha size: {}", a, alpha_size);

        // calculate new alphabet size,
        // break the loop in case it remains unchanged
        let new_alpha_size = 2 * log2_ceil(alpha_size);
        if alpha_size == new_alpha_size {
            break;
        }
        alpha_size = new_alpha_size;

        // reduce alphabet
        {
            // the following is equivalent to
            // for i in 1..a.len() {
            //     a[i-1] = label(a[i-1], a[i]);
            // }
            // but uses iterators to avoid array index bound checks.

            // creates iterator that yields mutable reference
            // to the elements
            let mut iter = a.iter_mut();

            // try to take first element, then iterate over all
            // remaining elements
            if let Some(mut i_minus_one_ptr) = iter.next() {
                for i_ptr in iter {
                    *i_minus_one_ptr = label(*i_minus_one_ptr, *i_ptr);
                    i_minus_one_ptr = i_ptr;
                }
            }
        }

        // remove unneeded last element
        len -= 1;
    }

    &mut a[..len]
}

/// return the least of {0,1,2} that is not in {a, b}
fn neighbor_check(a: Option<u8>, b: Option<u8>) -> u8 {
    if a != Some(0) && b != Some(0) {
        // no 0 on left or right
        0
    } else if a != Some(1) && b != Some(1) {
        // 0, but no 1 on left or right
        1
    } else {
        // 0 and 1 on left and right
        2
    }
}

fn phase2(a: &mut [u8]) {
    use std::mem;

    // replace all of 3, 4, 5
    for n in 3..6 {
        // iterate over slice, for each element compare
        // with both neighbors
        // for first and last element, compare with right/last neighbor only

        let mut iter = a.iter_mut();

        if let Some(current_ptr) = iter.next() {
            let mut left_ptr:    Option<&mut u8> = None;
            let mut current_ptr: &mut u8         = current_ptr;
            let mut right_ptr:   Option<&mut u8> = iter.next();

            loop {
                if *current_ptr == n {
                    *current_ptr = neighbor_check(
                        left_ptr.as_ref().map(|ptr| **ptr),
                        right_ptr.as_ref().map(|ptr| **ptr));
                }

                if let Some(right_ptr) = mem::replace(&mut right_ptr,
                                                      iter.next())
                {
                    left_ptr = Some(mem::replace(&mut current_ptr,
                                                 right_ptr));
                } else {
                    break
                }
            }
        }
    }
}

#[test]
fn test_lswr() {
    // Test example in paper by mapping a = 0, b = 1, c = 2 ...
    assert_eq!(lswr("cabageheadbag".chars()
                                   .map(|c| (c as u8) - b'a')
                                   .collect(), 8),
               vec![2, 1, 0, 1, 2, 1, 0, 2, 1, 2, 0, 1]);

    // Same string but in ascii/utf8 range
    assert_eq!(lswr("cabageheadbag".into(), 255),
               vec![1, 2, 0, 2, 1, 0, 2, 1, 0, 1]);

    assert_eq!(lswr("".into(), 255),
               vec![]);
}

#[test]
fn test_bit() {
    assert_eq!(bit(0, 255), 1);
    assert_eq!(bit(7, 255), 1);
    assert_eq!(bit(3, 255), 1);

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
fn test_lsb_differ_index() {
    assert_eq!(lsb_differ_index(0, 0), 8);
    assert_eq!(lsb_differ_index(0b0, 0b1), 0);
    assert_eq!(lsb_differ_index(0b00, 0b10), 1);
    assert_eq!(lsb_differ_index(0b100, 0b010), 1);
    assert_eq!(lsb_differ_index(0b1001, 0b0101), 2);
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
