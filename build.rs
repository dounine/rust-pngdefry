extern crate cc;

fn main() {
    let mut builder: cc::Build = cc::Build::new();

    let files = [
        // "pngdefry/miniz.c",
        "pngdefry/pngdefry.c",
    ];

    //依赖openssl
    builder
        .warnings(false)
        .flag("-c")
        .shared_flag(false)
        .files(files.iter())
        .compile("pngdefry");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=pngdefry/pngdefry.c");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings/bindings.rs")
        .expect("Couldn't write bindings!");
}
