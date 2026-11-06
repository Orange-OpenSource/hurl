[![CI Linux](https://github.com/KWARC/rust-libxml/actions/workflows/CI.yml/badge.svg?branch=master)](https://github.com/KWARC/rust-libxml/actions/workflows/CI.yml)
[![CI Windows](https://github.com/KWARC/rust-libxml/actions/workflows/windows.yml/badge.svg?branch=master)](https://github.com/KWARC/rust-libxml/actions/workflows/windows.yml)
[![API Documentation](https://img.shields.io/badge/docs-API-blue.svg)](http://KWARC.github.io/rust-libxml/libxml/index.html)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/KWARC/rust-libxml/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/libxml.svg)](https://crates.io/crates/libxml)

Rust wrapper for [libxml2](http://xmlsoft.org/).

The main goal of this project is to benefit from libxml2's maturity and stability while the native Rust XML crates mature to be near-drop-in replacements.

As of the `0.2.0` release of the crate, there are some modest safety guarantees:

 * Mutability, as well as ownership - we use `Rc<RefCell<T>>` wrappers to ensure runtime safety of libxml2 operations already in the Rust layer.
 * Memory safety guarantees - in particular `Node` and `Document` objects have automatic bookkeeping and deallocation on drop, for leak-free wrapper use.
 * No thread safety - libxml2's global memory management is a challenge to adapt in a thread-safe way with minimal intervention

**Coverage**: Only covers a subset of libxml2 at the moment, contributions are welcome. We try to increase support with each release.

**Welcome!** With these caveats, the contributors to the project are migrating production work towards Rust and find a continuing reliance on libxml2 a helpful relief for initial ports. As such, contributions to this crate are welcome, if your workflow is not yet fully supported.

## Installation prerequisites

Before performing the usual cargo build/install steps, you need to have the relevant components for using the original libxml2 code. These may become gradually outdated with time - please do let us know by opening a new issue/PR whenever that's the case.

### Linux/Debian

On linux systems you'd need the development headers of libxml2 (e.g. `libxml2-dev` in Debian), as well as `pkg-config`.

Starting with `0.3.4`, build requires `bindgen` (see [`bindgen` requirements](https://rust-lang.github.io/rust-bindgen/requirements.html#clang)).
So you have to install CLang 9.0 or greater:

- Alpine: `$ apk add clang-dev`
- Arch Linux: `$ pacman -S clang`
- Debian / Ubuntu: `$ apt install libclang-dev`
- Fedora: `$ dnf install clang-devel`

### Static linking (musl / Alpine / fully static binaries)

Two independent knobs build a binary with no runtime libxml2 / libclang
dependency (see [#110](https://github.com/KWARC/rust-libxml/issues/110)):

* **`static` cargo feature** — selects how `bindgen` finds libclang at build
  time. The default `runtime` feature `dlopen`s libclang, which is unavailable
  in fully static / musl environments; `static` links it via `clang-sys/static`
  instead:

  ```
  cargo build --no-default-features --features static
  ```

* **`LIBXML2_STATIC` env var** — statically links libxml2 *itself*. Point
  `PKG_CONFIG_PATH` at a prefix holding a PIC `libxml2.a` and set
  `LIBXML2_STATIC=1`; `build.rs` then emits `cargo:rustc-link-lib=static=xml2`,
  so the binary carries no `libxml2.so` SONAME dependency.

### MacOS
[Community contributed](https://github.com/KWARC/rust-libxml/issues/88#issuecomment-890876895):

```
$ brew install libxml2 # e.g. version 2.9.12 
$ ln -s /usr/local/Cellar/libxml2/2.9.12/lib/libxml2.2.dylib /usr/local/lib/libxml-2.0.dylib
$ export LIBXML2=/usr/local/Cellar/libxml2/2.9.12/lib/pkgconfig/libxml-2.0.pc
```

### FreeBSD
[Community contributed](https://github.com/KWARC/rust-libxml/issues/130#issuecomment-1976348349)

```
$ pkg install libxml2 pkgconf
```

### Windows

#### msvc

[Community contributed](https://github.com/KWARC/rust-libxml/issues/81#issuecomment-760364976):

* manually install builds tools c++ and english language by visiting [BuildTools](https://visualstudio.microsoft.com/fr/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
* launch cmd prompt with admin privileges and execute these commands sequentially:
```
C:\> git clone https://github.com/microsoft/vcpkg
C:\> .\vcpkg\bootstrap-vcpkg.bat
C:\> setx /M PATH "%PATH%;c:\vcpkg" && setx VCPKGRS_DYNAMIC "1" /M
C:\> refreshenv
C:\> vcpkg install libxml2:x64-windows
C:\> vcpkg integrate install
```

#### gnu

On mingw64 environment you could install libxml2 with `pacman -S mingw64/mingw-w64-x86_64-libxml2`.