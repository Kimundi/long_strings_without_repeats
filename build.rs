extern crate gcc;

fn main() {
    ::std::env::set_var("CXX", "clang");
    ::std::env::set_var("OPT_LEVEL", "3");

    gcc::Config::new()
        .cpp(true) // Switch to C++ library compilation.
        .flag("-std=c++11") // Use C++11 features
        .file("src/cpp_naive.cpp")
        .compile("libcpp_lswr.a");
}
