# highctidh-rs

Rust FFI bindings to the highctidh C library (CTIDH-1024), used by hpqc's NIKE wrapper.

`highctidh/` is codeberg.org/vula/highctidh v1.0.2026101300 (`src/ctidh1024`,
C sources only), built with `cc` in its portable C form. The crate exposes key
generation, Go's key encodings, validation, the shared secret and blinding;
`tests/ctidh1024.json` holds the Go vectors it matches byte for byte.

The crate is `no_std` (it needs `alloc`); the default `std` feature only
turns on `std` in its dependencies. Building for another target compiles the C
for that target, so it needs that target's C compiler and libc headers, e.g.
`gcc-arm-none-eabi` and `libnewlib-arm-none-eabi` for `thumbv7em-none-eabihf`.
On a target without an OS `randombytes.c` is left out and the final link must
provide `highctidh_1024_randombytes(void *, size_t)`.

To link a highctidh built elsewhere instead, such as the T-Deck firmware's
`kq_ctidh` component, set `HIGHCTIDH_LIB_DIR` to the directory holding the
static library (named by `HIGHCTIDH_LIB_NAME`, default `highctidh1024`), or
enable the `external` feature to link nothing and let the final link resolve
the `highctidh_1024_*` symbols. It must be the 1024-bit build (`BITS=1024`).
