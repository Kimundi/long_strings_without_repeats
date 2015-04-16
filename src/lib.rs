#![feature(core)]

fn lsb_differ_index(a: u8, b: u8) -> u8 {
    // xor => bits mit underschieden sind 1
    // trailing_zero => position des ersten unterschieds
    (a ^ b).trailing_zeros() as u8
}

fn bit(i: u8, byte: u8) -> u8 {
    //println!("{} {}", i, byte);
    if i > 7 {
        0
    } else {
        (byte >> i) & 1
    }
}

fn label(a_i_minus_one: u8, a_i: u8) -> u8 {
    let l = lsb_differ_index(a_i_minus_one, a_i);
    2 * l + bit(l, a_i)
}

fn alphabet_size(symbols: &[u8]) -> u8 {
    let mut alpha = [0u8; 256];
    for &i in symbols {
        alpha[i as usize] = 1;
    }
    alpha.iter().cloned().sum()
}

fn log2_ceil(n: u8) -> u8 {
    8 - (n-1).leading_zeros() as u8
}

pub fn lswr(mut a: Vec<u8>, mut alpha_size: u8) -> Vec<u8> {
    for j in 1..  {
        println!("{:?} alpha size: {}", a, alpha_size);

        let new_alpha_size = 2 * log2_ceil(alpha_size);
        if alpha_size == new_alpha_size {
            break;
        }
        alpha_size = new_alpha_size;

        for i in (j..a.len()).rev() {
            a[i] = label(a[i-1], a[i]);
        }

        for i in 0..j {
            a[i] = 255;
        }


    }

    a[0] = !0;
    a
}

#[test]
fn test_lswr() {
    // Test example in paper by mapping a = 0, b = 1, c = 2 ...
    assert_eq!(lswr("cabageheadbag".chars()
                                   .map(|c| (c as u8) - b'a')
                                   .collect(), 8),
                    vec![!0, 2, 1, 0, 1, 2, 1, 0, 2, 1, 2, 0, 1]);
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
