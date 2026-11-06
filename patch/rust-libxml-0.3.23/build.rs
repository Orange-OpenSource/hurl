use std::{
  env, fs,
  path::{Path, PathBuf},
};

struct ProbedLib {
  version: String,
  include_paths: Vec<PathBuf>,
}

/// Finds libxml2 and optionally return a list of header
/// files from which the bindings can be generated.
fn find_libxml2() -> Option<ProbedLib> {
  #![allow(unreachable_code)] // for platform-dependent dead code

  if let Ok(ref s) = std::env::var("LIBXML2") {
    // println!("{:?}", std::env::vars());
    // panic!("set libxml2.");
    let p = std::path::Path::new(s);
    let fname = std::path::Path::new(
      p.file_name()
        .unwrap_or_else(|| panic!("no file name in LIBXML2 env ({s})")),
    );
    assert!(
      p.is_file(),
      "{}",
      &format!("not a file in LIBXML2 env ({s})")
    );
    println!(
      "cargo:rustc-link-lib={}",
      fname
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .strip_prefix("lib")
        .unwrap()
    );
    println!(
      "cargo:rustc-link-search={}",
      p.parent()
        .expect("no library path in LIBXML2 env")
        .to_string_lossy()
    );
    None
  } else {
    #[cfg(any(
      target_family = "unix",
      target_os = "macos",
      all(target_family = "windows", target_env = "gnu")
    ))]
    {
      // Opt-in static link: when LIBXML2_STATIC is set and PKG_CONFIG_PATH points
      // at a non-system prefix carrying a PIC `libxml2.a`, `.statik(true)` makes
      // pkg-config emit `cargo:rustc-link-lib=static=xml2` plus the transitive
      // `-lm` (which stays dynamic). pkg-config's static guard refuses a `static=`
      // for a /usr/lib system path, so the non-system prefix is what enables this.
      // Unset (the default) is unchanged: a normal dynamic link against the host
      // libxml2. Used for the self-contained, SONAME-independent release binary.
      let mut cfg = pkg_config::Config::new();
      if std::env::var_os("LIBXML2_STATIC").is_some() {
        cfg.statik(true);
      }
      let lib = cfg
        .probe("libxml-2.0")
        .expect("Couldn't find libxml2 via pkg-config");
      return Some(ProbedLib {
        include_paths: lib.include_paths,
        version: lib.version,
      });
    }

    #[cfg(all(target_family = "windows", target_env = "msvc"))]
    {
      // Prefer pkg-config when available (e.g. conda-forge, MSYS-provided
      // pkg-config with an MSVC libxml2), so bindings are generated with
      // bindgen for the actual target. Falls back to vcpkg otherwise.
      // Set `LIBXML_2.0_NO_PKG_CONFIG=1` to skip pkg-config.
      match pkg_config::Config::new().probe("libxml-2.0") {
        Ok(lib) => {
          return Some(ProbedLib {
            include_paths: lib.include_paths,
            version: lib.version,
          });
        }
        Err(e) => eprintln!("pkg-config did not find libxml2 ({e}), trying vcpkg."),
      }
      if let Some(meta) = vcpkg_dep::vcpkg_find_libxml2() {
        return Some(meta);
      } else {
        eprintln!("vcpkg did not succeed in finding libxml2.");
      }
    }

    panic!("Could not find libxml2.")
  }
}

fn generate_bindings(header_dirs: Vec<PathBuf>, output_path: &Path) {
  let bindings = bindgen::Builder::default()
    .header("src/wrapper.h")
    .opaque_type("max_align_t")
    // invalidate build as soon as the wrapper changes
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .layout_tests(true)
    .clang_args(&[
      "-DPKG-CONFIG",
      "-DLIBXML_C14N_ENABLED",
      "-DLIBXML_OUTPUT_ENABLED",
    ])
    .clang_args(header_dirs.iter().map(|dir| format!("-I{}", dir.display())));
  bindings
    .generate()
    .expect("failed to generate bindings with bindgen")
    .write_to_file(output_path)
    .expect("Failed to write bindings.rs");
}

fn main() {
  // Re-run if the opt-in static toggle flips (see find_libxml2's static branch).
  println!("cargo:rerun-if-env-changed=LIBXML2_STATIC");
  let bindings_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs");
  // declare availability of config variable (without setting it)
  println!("cargo::rustc-check-cfg=cfg(libxml_older_than_2_12)");

  if let Some(probed_lib) = find_libxml2() {
    // if we could find header files, generate fresh bindings from them
    generate_bindings(probed_lib.include_paths, &bindings_path);
    // and expose the libxml2 version to the code
    let version_parts: Vec<i32> = probed_lib
      .version
      .split('.')
      .map(|part| part.parse::<i32>().unwrap_or(-1))
      .collect();
    let older_than_2_12 = version_parts.len() > 1
      && (version_parts[0] < 2 || version_parts[0] == 2 && version_parts[1] < 12);
    println!("cargo::rustc-check-cfg=cfg(libxml_older_than_2_12)");
    if older_than_2_12 {
      println!("cargo::rustc-cfg=libxml_older_than_2_12");
    }
  } else {
    // otherwise, use the default bindings on platforms where pkg-config isn't available
    fs::copy(PathBuf::from("src/default_bindings.rs"), bindings_path)
      .expect("Failed to copy the default bindings to the build directory");
    // for now, assume that the library is older than 2.12, because that's what those bindings are computed with
    println!("cargo::rustc-cfg=libxml_older_than_2_12");
  }
}

#[cfg(all(target_family = "windows", target_env = "msvc"))]
mod vcpkg_dep {
  use crate::ProbedLib;
  pub fn vcpkg_find_libxml2() -> Option<ProbedLib> {
    if let Ok(metadata) = vcpkg::Config::new().find_package("libxml2") {
      // vcpkg emits link directives for the port's own libraries and its
      // port dependencies (iconv, zlib, ...), but NOT for Windows SDK
      // system libraries, which are Libs.private in libxml-2.0.pc:
      // libxml2 >= 2.12 calls BCryptGenRandom (xmlInitRandom, dict.c), so
      // a static libxml2.lib leaves that symbol unresolved unless we link
      // bcrypt ourselves. ws2_32 likewise backs the (default-on) nanohttp
      // code. Both import libraries ship with every MSVC/Windows SDK
      // toolchain, so linking them unconditionally here is safe even for
      // vcpkg builds configured without those features.
      println!("cargo:rustc-link-lib=bcrypt");
      println!("cargo:rustc-link-lib=ws2_32");
      let include_paths = metadata
        .include_paths
        .into_iter()
        .fold(Vec::new(), |mut acc, p| {
          acc.push(p.join("libxml2"));
          acc.push(p);
          acc
        });
      return Some(ProbedLib {
        version: vcpkg_version(),
        include_paths,
      });
    }
    None
  }

  fn vcpkg_version() -> String {
    // What is the best way to obtain the version on Windows *before* bindgen runs?
    // here we attempt asking the shell for "vcpkg list libxml2"
    let mut vcpkg_exe = vcpkg::find_vcpkg_root(&vcpkg::Config::new()).unwrap();
    vcpkg_exe.push("vcpkg.exe");
    let vcpkg_list_libxml2 = std::process::Command::new(vcpkg_exe)
      .args(["list", "libxml2"])
      .output()
      .expect("vcpkg.exe failed to execute in vcpkg_dep build step");
    if vcpkg_list_libxml2.status.success() {
      let libxml2_list_str = String::from_utf8_lossy(&vcpkg_list_libxml2.stdout);
      for line in libxml2_list_str.lines() {
        if line.starts_with("libxml2:") {
          let mut version_piece = line.split("2.");
          version_piece.next();
          if let Some(version_tail) = version_piece.next() {
            if let Some(version) = version_tail.split(' ').next().unwrap().split('#').next() {
              return format!("2.{version}");
            }
          }
        }
      }
    }
    // default to a recent libxml2 from Windows 10
    // (or should this panic?)
    String::from("2.13.5")
  }
}
