fn main() {
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=c");
}
