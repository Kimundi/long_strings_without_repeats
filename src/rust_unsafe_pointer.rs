use bit;
use log2_ceil;
use lsb_differ_index;

/// longest strings without repeats algorithm
pub fn lswr(a: &mut [u8], alpha_size: u8) -> &mut [u8] {
    let new_len = phase1(a.as_mut_ptr(), a.len(), alpha_size);
    let a = &mut a[..new_len];
    phase2(a.as_mut_ptr(), a.len());
    a
}

/// alphabet reduction label calculation
fn label(a_i_minus_one: u8, a_i: u8) -> u8 {
    let l = lsb_differ_index(a_i_minus_one, a_i);
    2 * l + bit(l, a_i)
}

/// first phase, reducing until alphabet size unchanged and == 6
fn phase1(mut a: *mut u8, a_len: usize, mut alpha_size: u8) -> usize {
    let mut len = a_len;

    macro_rules! a {
        ($i:expr) => (*a.offset($i as isize))
    }

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
                a![i-1] = label(a![i-1], a![i]);
            }
        }

        // remove unneeded last element
        len -= 1;
    }

    len
}

/// return the least of {0, 1, 2} that is not in {a, b}
fn neighbor_check(a: i32, b: i32) -> u8 {
    if a != 0 && b != 0 {
        // no 0 on left or right
        0
    } else if a != 1 && b != 1 {
        // 0, but no 1 on left or right
        1
    } else {
        // 0 and 1 on left and right
        2
    }
}

/// second phase, reducing alphabet from 6 to 3
fn phase2(a: *mut u8, a_len: usize) {
    macro_rules! a {
        ($i:expr) => (*a.offset($i as isize))
    }

    // replace all of 3, 4, 5
    for n in 3..6 {
        // iterate over slice, for each element compare
        // with both neighbors
        // for first and last element, compare with right/last neighbor only

        for i in 0..a_len {
            unsafe {
                if a![i] == n {
                    let left  = if i > 0         { a![i-1] as i32 } else { -1 };
                    let right = if i < a_len - 1 { a![i+1] as i32 } else { -1 };
                    a![i] = neighbor_check(left, right);
                }
            }
        }
    }
}
