extern crate gcc;

fn main() {
    gcc::compile_library("libmacproc.a", &["src/macproc.c"]);    
    println!("cargo:rustc-link-lib=macproc");
}
