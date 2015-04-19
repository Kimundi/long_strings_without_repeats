# Long Strings Without Repeats

Implementations in both Rust and C++

Assuming both Rust and clang is installed, and the repo cloned locally,
the testsuite and benchmarks
can be run like this:

```
cargo test
cargo bench
```

Sample output:

```
[...]

     Running target/debug/long_strings_without_repeats-db470b5cc4c0e52e

running 7 tests
test rust_naive::test_phase2 ... ok
test rust_naive::test_phase1 ... ok
test rust_unsafe::test_phase1 ... ok
test rust_unsafe::test_phase2 ... ok
test test_bit ... ok
test test_log2_ceil ... ok
test test_lsb_differ_index ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/test-cf364adb153d564e

running 4 tests
test test_lswr_rust_unsafe ... ok
test test_lswr_cpp_naive ... ok
test test_lswr_rust_naive ... ok
test test_comparison ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured

[...]

     Running target/release/bench-4de3e1ccaf3d54e1

running 8 tests
test bench_cpp_naive_big     ... bench:  10331621 ns/iter (+/- 116854)
test bench_cpp_naive_short   ... bench:       124 ns/iter (+/- 1)
test bench_noop_big          ... bench:   1329028 ns/iter (+/- 40725)
test bench_noop_short        ... bench:        20 ns/iter (+/- 1)
test bench_rust_naive_big    ... bench:  16501898 ns/iter (+/- 757182)
test bench_rust_naive_short  ... bench:       170 ns/iter (+/- 7)
test bench_rust_unsafe_big   ... bench:  13891298 ns/iter (+/- 134482)
test bench_rust_unsafe_short ... bench:       153 ns/iter (+/- 3)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured

[...]

```
