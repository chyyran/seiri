extern crate cmake;
extern crate bindgen;

use std::path::PathBuf;
use std::env;

fn main() {
  let dst = cmake::Config::new("libkatatsuki")
            .build_target("katatsuki")
            .build();
  let profile = env::var("PROFILE").unwrap();
  let mut lib_dst = PathBuf::from(format!("{}", dst.display()));
  lib_dst.push("build");
  lib_dst.push(profile);
  println!("cargo:rustc-link-search=native={}", lib_dst.display());
  println!("cargo:rustc-link-lib=static=katatsuki");

  // let bindings = bindgen::Builder::default()
  //   .header("wrapper.h")
  //   .generate()
  //   .expect("Unable to generate bindings");

  // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  // bindings
  //   .write_to_file(out_path.join("bindings.rs"))
  //   .expect("Couldn't write bindings!");
}
