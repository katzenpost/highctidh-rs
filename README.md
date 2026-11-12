# highctidh-rs

Rust bindings to CTIDH-1024 from the highctidh C library, with the key and
shared-secret encodings of hpqc's Go `ctidh1024` package. It is for Rust code
that has to exchange CTIDH-1024 keys with
[hpqc](https://github.com/katzenpost/hpqc), such as a Rust port of hpqc's NIKE
wrapper.

By default the crate builds a vendored copy of highctidh with `cc`; it offers key
generation, key encoding and validation, the shared secret, and blinding
(the group action on a public key). It covers CTIDH-1024 only; for the other
CTIDH sizes or for Go, use highctidh or hpqc directly. It is `no_std` (it
needs `alloc`) and builds for bare-metal targets such as
`thumbv7em-none-eabihf`. It is not on crates.io.

## Quick start

You need Rust 1.88 or later and a C compiler.

```toml
[dependencies]
highctidh = { git = "https://github.com/katzenpost/highctidh-rs" }
rand_core = "0.10"
getrandom = { version = "0.4", features = ["sys_rng"] }
```

```rust
use highctidh::{PrivateKey, PublicKey};

fn main() {
    let mut rng = rand_core::UnwrapErr(getrandom::SysRng);
    let alice = PrivateKey::generate(&mut rng);
    let bob = PrivateKey::generate(&mut rng);

    // 128-byte public keys, in the encoding hpqc's Go ctidh1024 uses
    let bob_pk = PublicKey::from_bytes(&bob.public_key().to_bytes()).unwrap();
    let alice_pk = alice.public_key();

    let secret = alice.shared_secret(&bob_pk).unwrap();
    assert_eq!(*secret, *bob.shared_secret(&alice_pk).unwrap());
}
```

## Contents

- [`src/lib.rs`](src/lib.rs): `PrivateKey`, `PublicKey` and `Error`.
- [`src/ffi.rs`](src/ffi.rs): the calls into the C library.
- [`build.rs`](build.rs): compiles the C sources with the flags highctidh's cgo binding uses, in its portable C form.
- [`bare-metal/endian.h`](bare-metal/endian.h): the `<endian.h>` newlib lacks, used when the target has no OS.
- [`highctidh/`](highctidh): the C sources of [codeberg.org/vula/highctidh](https://codeberg.org/vula/highctidh) v1.0.2026101300, `src/ctidh1024` only.
- [`tests/go_vectors.rs`](tests/go_vectors.rs) and [`tests/ctidh1024.json`](tests/ctidh1024.json): hpqc's Go test vectors.

## Features and other targets

The default `std` feature only turns on `std` in the dependencies; with
`default-features = false` the crate needs only `alloc`. Building for another
target compiles the C for that target, so it needs that target's C compiler
and libc headers, e.g. `gcc-arm-none-eabi` and `libnewlib-arm-none-eabi` for
`thumbv7em-none-eabihf`:

```
cargo build --no-default-features --target thumbv7em-none-eabihf
```

On a target without an OS `randombytes.c` is left out, and the final link must
provide `highctidh_1024_randombytes(void *, size_t)`.

To link a highctidh built elsewhere instead, such as the T-Deck firmware's
`kq_ctidh` component, set `HIGHCTIDH_LIB_DIR` to the directory holding the
static library (named by `HIGHCTIDH_LIB_NAME`, default `highctidh1024`), or
enable the `external` feature to link nothing and let the final link resolve
the `highctidh_1024_*` symbols. It must be the 1024-bit build (`BITS=1024`).

## Compatibility

Public keys and shared secrets are 128 bytes and private keys 130 bytes,
encoded as hpqc's Go `ctidh1024` encodes them. Given the same random bytes,
`PrivateKey::generate` returns the same key as Go's
`GenerateKeyPairFromEntropy`. `tests/ctidh1024.json` holds the `ctidh1024`
cases of hpqc's `testvectors/nike/schemes.json`; the crate matches their keys,
shared secrets and blinded keys byte for byte.

## Testing and contributing

```
cargo test
cargo test --no-default-features
```

CI runs both and the `thumbv7em-none-eabihf` build above.

Issues and pull requests are welcome on GitHub.

## Licence

Public domain, as is highctidh; see [LICENSE](LICENSE).
