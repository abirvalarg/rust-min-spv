fn main() {
    cc::Build::new()
        .file("src/init.c")
        .file("src/volatile.c")
        .file("src/asm.c")
        .compile("c-part.a");
}
