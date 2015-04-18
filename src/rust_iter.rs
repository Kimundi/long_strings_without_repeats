use bit;
use log2_ceil;
use lsb_differ_index;

/// longest strings without repeats algorithm
pub fn lswr(a: &mut [u8], alpha_size: u8) -> &mut [u8] {
    let new_len = phase1(a, alpha_size);
    phase2(a);

    &mut a[..new_len]
}

/// alphabet reduction label calculation
fn label(a_i_minus_one: u8, a_i: u8) -> u8 {
    let l = lsb_differ_index(a_i_minus_one, a_i);
    2 * l + bit(l, a_i)
}

/// first phase, reducing until alphabet size unchanged and == 6
fn phase1(mut a: &mut [u8], mut alpha_size: u8) -> usize {
    let mut len = a.len();

    while len > 0 {
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

    len
}

/// return the least of {0, 1, 2} that is not in {a, b}
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

/// second phase, reducing alphabet from 6 to 3
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
fn test_phase1() {
    // Compare with paper example
    let mut string = ::new_paper_example_string();
    let new_len = phase1(&mut string, 8);
    assert_eq!(&mut string[..new_len],
                &mut [2, 1, 0, 3, 2, 1, 0, 4, 1, 2, 0, 3]);
}

#[test]
fn test_phase2() {
    // Compare with paper example
    let mut string = vec![2, 1, 0, 3, 2, 1, 0, 4, 1, 2, 0, 3];
    phase2(&mut string);
    assert_eq!(string,
                vec![2, 1, 0, 1, 2, 1, 0, 2, 1, 2, 0, 1]);
}
