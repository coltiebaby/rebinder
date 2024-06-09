use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path =
        std::fs::canonicalize("../remixer/src/").expect("could not get path to remixer");
    let libdir_path_str = libdir_path.to_str().expect("Path is not a valid string");

    let header_path = libdir_path.join("remixer.h");
    let header_path_str = header_path.to_str().expect("Path is not a valid string");

    let c_path = libdir_path.join("remixer.c");
    let obj_path = libdir_path.join("remixer.o");
    let lib_path = libdir_path.join("libremixer.a");

    println!("cargo:rustc-link-search={}", libdir_path_str);
    println!("cargo:rustc-link-lib=remixer");

    // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("clang")
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&obj_path)
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    // Run `ar` to generate the `libhello.a` file from the `hello.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(&lib_path)
        .arg(&obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }

    let bindings = bindgen::Builder::default()
        .header(header_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}
