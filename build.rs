extern crate gcc;

fn main() {
    ::std::env::set_var("CXX", "clang");

    gcc::Config::new()
        .cpp(true) // Switch to C++ library compilation.
        .flag("-std=c++11") // Use C++11 features
        .file("src/cpp_naive.cpp")
        .compile("libcpp_naive.a");
}
