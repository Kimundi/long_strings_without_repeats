extern crate libc;
use self::libc::{uintptr_t, uint8_t};

extern {
    fn lswr_naive_cpp_ffi(a: *mut uint8_t,
                          a_len: uintptr_t,
                          alpha_size: uint8_t) -> uintptr_t;
}

/// longest strings without repeats algorithm
pub fn lswr(a: &mut [u8], alpha_size: u8) -> &mut [u8] {
    unsafe {
        let new_len = lswr_naive_cpp_ffi(a.as_mut_ptr() as *mut uint8_t,
                                         a.len()        as uintptr_t,
                                         alpha_size     as uint8_t);
        &mut a[..new_len as usize]
    }
}
