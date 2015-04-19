use bit;
use log2_ceil;
use lsb_differ_index;

/// longest strings without repeats algorithm
pub fn lswr(a: &mut [u8], alpha_size: u8) -> &mut [u8] {
    let new_len = phase1(a, alpha_size);
    let a = &mut a[..new_len];
    phase2(a);
    a
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
        for i in 1..len {
            unsafe {
                *a.get_unchecked_mut(i - 1) = label(*a.get_unchecked(i - 1),
                                                    *a.get_unchecked(i));
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
    // replace all of 3, 4, 5
    for n in 3..6 {
        // iterate over slice, for each element compare
        // with both neighbors
        // for first and last element, compare with right/last neighbor only

        for i in 0..a.len() {
            unsafe {
                if *a.get_unchecked(i) == n {
                    let left  = if i > 0           { Some(*a.get_unchecked(i-1)) } else { None };
                    let right = if i < a.len() - 1 { Some(*a.get_unchecked(i+1)) } else { None };
                    *a.get_unchecked_mut(i) = neighbor_check(left, right);
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
