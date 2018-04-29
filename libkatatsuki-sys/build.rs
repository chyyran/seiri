fn main() {
  println!("cargo:rustc-flags=-l ole32");
  println!("cargo:rustc-link-search=native=./lib");
}
