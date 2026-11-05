# highctidh-rs

Rust FFI bindings to the highctidh C library (CTIDH-1024), used by hpqc's NIKE wrapper.

`highctidh/` is codeberg.org/vula/highctidh v1.0.2026101300 (`src/ctidh1024`,
C sources only), built with `cc` in its portable C form. The crate exposes key
generation, Go's key encodings, validation, the shared secret and blinding;
`tests/ctidh1024.json` holds the Go vectors it matches byte for byte.
