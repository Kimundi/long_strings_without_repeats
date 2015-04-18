extern crate libc;
use self::libc::{c_char, size_t};

extern {
    fn lswr_cpp_ffi(a: *mut c_char, a_len: size_t, alpha_size: c_char) -> size_t;
}

/// longest strings without repeats algorithm
pub fn lswr(a: &mut [u8], alpha_size: u8) -> &mut [u8] {
    unsafe {
        let new_len = lswr_cpp_ffi(a.as_mut_ptr() as *mut c_char,
                                   a.len() as size_t,
                                   alpha_size as c_char);
        &mut a[..new_len as usize]
    }
}
