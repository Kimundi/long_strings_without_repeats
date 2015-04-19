# Long Strings Without Repeats

Implementations in both Rust and C++

Assuming both Rust and clang is installed, and the repo cloned locally,
the testsuite and benchmarks
can be run like this:

```
cargo test
cargo bench
```

Sample benchmark results:

```
test bench_cpp_naive_big     ... bench:  10331621 ns/iter (+/- 116854)
test bench_cpp_naive_short   ... bench:       124 ns/iter (+/- 1)
test bench_noop_big          ... bench:   1329028 ns/iter (+/- 40725)
test bench_noop_short        ... bench:        20 ns/iter (+/- 1)
test bench_rust_naive_big    ... bench:  16501898 ns/iter (+/- 757182)
test bench_rust_naive_short  ... bench:       170 ns/iter (+/- 7)
test bench_rust_unsafe_big   ... bench:  13891298 ns/iter (+/- 134482)
test bench_rust_unsafe_short ... bench:       153 ns/iter (+/- 3)
```
