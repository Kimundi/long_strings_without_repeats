extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true) // Switch to C++ library compilation.
        .file("src/cpp_index.cpp")
        .flag("-std=c++11")
        .compile("libcpp_index.a");
}
