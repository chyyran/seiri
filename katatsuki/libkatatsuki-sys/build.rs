extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
  let dst = if cfg!(target_os = "windows") {
    cmake::Config::new("libkatatsuki")
            //  .generator("NMake Makefiles")
              .build_target("katatsuki")
              .static_crt(true)
              .cxxflag("/MT")
              .cflag("/MT")
              .cxxflag("/NODEFAULTLIB:MSVCRT")
              .always_configure(true)
              .profile("Release")
              .very_verbose(true)
              .build()
    } else {
      cmake::Config::new("libkatatsuki")
        .build_target("katatsuki")
        .profile("Release")
        .always_configure(true)
        .very_verbose(true)
        .cxxflag("-std=c++1z")
        .build()
    };
  //let profile = env::var("PROFILE").unwrap();
  let mut lib_dst = PathBuf::from(format!("{}", dst.display()));
  let mut taglib_dst = PathBuf::from(format!("{}", dst.display()));

  lib_dst.push("build");

  if cfg!(target_os = "windows") {
    lib_dst.push("Release");
  }

  taglib_dst.push("build");
  taglib_dst.push("taglib");
  taglib_dst.push("lib");

  println!("cargo:rustc-link-search=native={}", lib_dst.display());
  println!("cargo:rustc-link-search=native={}", taglib_dst.display());
  println!("cargo:rustc-link-lib=static=tag");
  println!("cargo:rustc-link-lib=static=tag_c");
  println!("cargo:rustc-link-lib=static=katatsuki");

  // let bindings = bindgen::Builder::default()
  //   .header("wrapper.h")
  //   .generate()
  //   .expect("Unable to generate bindings");

 //  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  // bindings
  //   .write_to_file(out_path.join("bindings.rs"))
  //   .expect("Couldn't write bindings!");
}
