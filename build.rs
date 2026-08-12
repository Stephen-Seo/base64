fn main() {
    println!("cargo::rustc-link-arg-cdylib=-Wl,-soname,libbase64.so.0");
}
