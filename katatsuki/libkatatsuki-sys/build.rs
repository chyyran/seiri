extern crate semver;

use semver::Version;

use std::env;
use std::process::Command;
use std::fs::remove_dir_all;
use std::path::PathBuf;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
static runtime_identifier: &'static str = "linux-x64";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
static runtime_identifier: &'static str = "osx-x64";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
static runtime_identifier: &'static str = "win-x64";

#[cfg(not(target_arch = "x86_64"))]
panic!("libkatatsuki can only be built in a x86_64 environment");

fn main() {
  println!("cargo:rustc-env=DOTNET_CLI_TELEMETRY_OPTOUT=1");
  if get_dotnet_command().spawn().is_err() {
    panic!("Unable to find .NET Core SDK. Install the .NET Core SDK from https://dot.net")
  }
  let path = get_out_path();
  dotnet_version_sufficient();
  dotnet_do_restore();
  dotnet_do_clean();
  let profile = env::var("PROFILE").unwrap();
  #[cfg(all(target_os = "windows", target_arch = "x86_64"))] { dotnet_do_build_shared(&profile, &path); }
  dotnet_do_build_static(&profile, &path);
  #[cfg(all(target_os = "windows", target_arch = "x86_64"))] { println!("cargo:rustc-flags=-l ole32"); }
  println!("cargo:rustc-link-search=native={}", path);
  println!("cargo:rustc-link-search=native={}", get_sdk_path().display());
  #[cfg(all(target_os = "windows", target_arch = "x86_64"))] { println!("cargo:rustc-link-lib=static=libkatatsuki"); }
  #[cfg(all(target_os = "linux", target_arch = "x86_64"))] { println!("cargo:rustc-link-lib=static=katatsuki"); }
  #[cfg(all(target_os = "macos", target_arch = "x86_64"))] { println!("cargo:rustc-link-lib=static=katatsuki"); }
  println!("cargo:rustc-link-lib=static=bootstrapperdll");
  println!("cargo:rustc-link-lib=static=Runtime");

}

fn get_sdk_path() -> PathBuf {
  let mut home_dir = env::home_dir().unwrap();
  home_dir.push(".nuget");
  home_dir.push("packages");
  home_dir.push(format!("runtime.{}.microsoft.dotnet.ilcompiler", runtime_identifier));
  home_dir.push("1.0.0-alpha-26428-01");
  home_dir.push("sdk");
  home_dir
}
#[allow(dead_code)]
fn get_out_path() -> String {
  format!("{}/libkatatsuki-sys", env::var("OUT_DIR").unwrap()).to_owned()
}

fn get_dotnet_command() -> Command {
  Command::new("dotnet")
}

fn dotnet_version_sufficient() {
  let dotnet_ver = String::from_utf8(
    get_dotnet_command()
      .args(&["--version"])
      .output()
      .unwrap()
      .stdout,
  ).unwrap();
  let dotnet_ver = Version::parse(&dotnet_ver).unwrap();
  if dotnet_ver < Version::parse("2.1.300-preview2-008533").unwrap() {
    panic!(".NET Core SDK. {} is insufficient. libkatatsuki requires at least .NET Core SDK 2.1.300-preview2-008533", dotnet_ver)
  }
}

fn dotnet_do_restore() {
  let status = get_dotnet_command()
    .arg("restore")
    .arg("./libkatatsuki/src")
    .status()
    .expect(
      "Unable to do first-time restore of NuGet packages for libkatatsuki.
             An internet connection is required for first build.",
    );
  if !status.success() {
    panic!(
      "Unable to do first-time restore of NuGet packages for libkatatsuki.
             An internet connection is required for first build."
    );
  }
}

fn dotnet_do_clean() {
  let status = get_dotnet_command()
    .arg("clean")
    .arg("./libkatatsuki/src")
    .status();
  if let Ok(status) = status {
    if !status.success() {
      println!("cargo:warning=Unable to clean build artifacts directory.");
    }
  } else {
    println!("cargo:warning=Unable to clean build artifacts directory.");
  }
}

fn dotnet_do_build_shared(profile: &str, path: &str) {
  let status = get_dotnet_command()
    .arg("publish")
    .arg("./libkatatsuki/src")
    .arg("/t:LinkNative")
    .arg("/p:NativeLib=Shared")
    .arg("-c")
    .arg(profile)
    .arg("-o")
    .arg(path)
    .arg("-r")
    .arg(runtime_identifier)
    .status()
    .expect("Unable to build shared version of libkatatsuki");
  if !status.success() {
    panic!("Unable to build shared version of libkatatsuki");
  }
}

fn dotnet_do_build_static(profile: &str, path: &str) {
  let status = get_dotnet_command()
    .arg("publish")
    .arg("./libkatatsuki/src")
    .arg("/t:LinkNative")
    .arg("/p:NativeLib=Static")
    .arg("-c")
    .arg(profile)
    .arg("-o")
    .arg(path)
    .arg("-r")
    .arg(runtime_identifier)
    .status()
    .expect("Unable to do build static version of libkatatsuki");
  if !status.success() {
    panic!("Unable to do build static version of libkatatsuki")
  }
}
