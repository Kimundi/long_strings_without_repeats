#include <cstddef>
#include <iostream>
#include <cstdint>
#include <climits>

using namespace::std;

static uint8_t leading_zeros(uint8_t n) {
    // Not aware of a more portable way right now,
    // this function works for gcc and clang on my system.

    // __builtin_clz() is defined for unsigned int, but we just
    // work with a byte, so we need to ignore the leading bytes.
    const int uncounted_bits = (sizeof(unsigned int) - sizeof(uint8_t)) * CHAR_BIT;
    return __builtin_clz(n) - uncounted_bits;
}

static uint8_t trailing_zeros(uint8_t n) {
    // Not aware of a more portable way right now,
    // this function works for gcc and clang on my system.

    return __builtin_ctz(n);
}

/// returns bit at index i
static uint8_t bit(uint8_t i, uint8_t byte) {
    return (byte >> i) & 1;
}

/// Returns rounded up binary logarithm of n, aka `ceil(log2(n))`.
/// This is only defined for n >= 1.
static uint8_t log2_ceil(uint8_t n) {
    // see http://en.wikipedia.org/wiki/Binary_logarithm#Integer_rounding
    return 8 - leading_zeros(n - 1);
}

/// Return the index of the least significant bit that differs
static uint8_t lsb_differ_index(uint8_t a, uint8_t b) {
    // xor => bits with differences are 1
    // trailing_zero => position of first difference
    return uint8_t(trailing_zeros(a ^ b));
}

/// alphabet reduction label calculation
static uint8_t label(uint8_t a_i_minus_one, uint8_t a_i) {
    uint8_t l = lsb_differ_index(a_i_minus_one, a_i);
    return 2 * l + bit(l, a_i);
}

/// first phase, reducing until alphabet size unchanged and == 6
/// the return type represents the length of the subarray containing the
/// final result
static uintptr_t phase1(uint8_t * a, uintptr_t a_len, uint8_t alpha_size) {
    uintptr_t len = a_len;

    while (len > 0) {
        // calculate new alphabet size,
        // break the loop in case it remains unchanged
        uint8_t new_alpha_size = 2 * log2_ceil(alpha_size);
        if (alpha_size == new_alpha_size) {
            break;
        }
        alpha_size = new_alpha_size;

        // reduce alphabet
        for (uintptr_t i = 1; i < len; i++) {
            a[i-1] = label(a[i-1], a[i]);
        }

        // remove unneeded last element
        len -= 1;
    }

    return len;
}

/// return the least of {0, 1, 2} that is not in {a, b}
/// (using int as a replacement for Option<u8> from the Rust version)
static uint8_t neighbor_check(int a, int b) {
    if (a != 0 && b != 0) {
        // no 0 on left or right
        return 0;
    } else if (a != 1 && b != 1) {
        // 0, but no 1 on left or right
        return 1;
    } else {
        // 0 and 1 on left and right
        return 2;
    }
}

/// second phase, reducing alphabet from 6 to 3
static void phase2(uint8_t * a, uintptr_t a_len) {
    // replace all of 3, 4, 5
    for (uint8_t n = 3; n < 6; n++) {
        // iterate over slice, for each element compare
        // with both neighbors
        // for first and last element, compare with right/last neighbor only

        for (uintptr_t i = 0; i < a_len; i++) {
            if (a[i] == n) {
                int left  = i > 0         ? a[i-1] : -1;
                int right = i < a_len - 1 ? a[i+1] : -1;
                a[i] = neighbor_check(left, right);
            }
        }
    }
}

/// longest strings without repeats algorithm
static uintptr_t lswr(uint8_t * a, uintptr_t a_len, uint8_t alpha_size) {
    uintptr_t new_len = phase1(a, a_len, alpha_size);
    phase2(a, new_len);

    return new_len;
}

/// C FFI wrapper that gets called by Rust
extern "C" uintptr_t lswr_cpp_ffi(uint8_t * a, uintptr_t a_len, uint8_t alpha_size) {
    return lswr(a, a_len, alpha_size);
}
