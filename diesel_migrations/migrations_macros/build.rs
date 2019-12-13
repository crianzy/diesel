
use std::collections::HashMap;
use std::env;

fn main() {
    println!("cargo:rustc-link-search={}", "/Users/chenzhiyong/Documents/bytedance/rocket/rust-sdk/deps/wcdb/android/armv7");
    println!("cargo:rustc-link-lib=wcdb");
}
