use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Command {
    directory: String,
    command: String,
    file: String,
}

fn main() {
    let dst = cmake::Config::new("vendor/ecl")
        .build_target("matrix")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .build();

    let outdir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let matrixdir = dst.join("build/matrix-prefix/src/matrix");
    let ekfdir = std::path::Path::new("vendor/ecl/EKF")
        .canonicalize()
        .unwrap();
    let ecldir = std::path::Path::new("vendor/ecl").canonicalize().unwrap();

    let jsontxt = std::fs::read_to_string(dst.join("build/compile_commands.json"))
        .expect("can't read compile_commands.json");
    let commands: Vec<Command> =
        serde_json::from_str(&jsontxt).expect("can't parse compile_commands.json");

    let mut builder = cc::Build::new();

    for command in commands {
        if command.file.contains("tests") {
            continue;
        }

        builder.file(command.file);
    }

    let vwrap_src = outdir.join("vwrap.cpp");
    let vwrap_hdr = outdir.join("vwrap.h");
    cpp_vwrap_gen::generate(
        &vwrap_src,
        &vwrap_hdr,
        &ekfdir.join("ekf.h"),
        &[
            "-x",
            "c++",
            "-std=c++14",
            "-DECL_STANDALONE",
            "-Wno-pragma-once-outside-header",
            &format!("-I{}", matrixdir.display()),
            &format!("-I{}", ecldir.display()),
            &format!("-I{}", ekfdir.display()),
        ],
    )
    .unwrap();
    builder.file(vwrap_src);

    builder.file(std::path::Path::new("helpers.cpp"));

    builder
        .warnings(true)
        .extra_warnings(true)
        .warnings_into_errors(true)
        .include(&matrixdir)
        .include(&ecldir)
        .include(&ekfdir)
        .include(".")
        .flag("-xc++")
        .flag("-std=c++14")
        .define("ECL_STANDALONE", None)
        .compile("px4-ecl");

    println!("cargo:rustc-link-search=native={}", outdir.display());
    println!("cargo:rustc-link-lib=px4-ecl");
    println!("cargo:rustc-link-lib=stdc++");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .clang_arg("-DECL_STANDALONE")
        .clang_arg(format!("-I{}", matrixdir.display()))
        .clang_arg(format!("-I{}", ecldir.display()))
        .clang_arg(format!("-I{}", ekfdir.display()))
        .clang_arg(format!("-I{}", outdir.display()))
        .layout_tests(true)
        .derive_default(true)
        .whitelist_type("Ekf")
        .whitelist_function("vwrap_.*")
        .whitelist_function("px4_ecl_sys_helper_.*")
        .opaque_type("matrix::.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(outdir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
