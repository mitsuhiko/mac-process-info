fn main() {
    cc::Build::new()
        .file("src/macproc.c")
        .compile("macproc");
}
