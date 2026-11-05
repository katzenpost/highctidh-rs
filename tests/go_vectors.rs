use blake2::Blake2b512;
use highctidh::{PrivateKey, PublicKey};
use hkdf::SimpleHkdf;
use rand_core::TryRng;

/// The deterministic HKDF-BLAKE2b-512 stream hpqc's Go vector generator
/// draws keys from.
struct HkdfRng {
    stream: Vec<u8>,
    pos: usize,
}

impl HkdfRng {
    fn new(label: &str, case: &str) -> Self {
        let secret = format!("hpqc-vector-seed-{label}");
        let mut stream = vec![0; 255 * 64];
        SimpleHkdf::<Blake2b512>::new(None, secret.as_bytes())
            .expand(case.as_bytes(), &mut stream)
            .unwrap();
        Self { stream, pos: 0 }
    }
}

impl TryRng for HkdfRng {
    type Error = core::convert::Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut b = [0; 4];
        self.try_fill_bytes(&mut b)?;
        Ok(u32::from_le_bytes(b))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut b = [0; 8];
        self.try_fill_bytes(&mut b)?;
        Ok(u64::from_le_bytes(b))
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        dst.copy_from_slice(&self.stream[self.pos..self.pos + dst.len()]);
        self.pos += dst.len();
        Ok(())
    }
}

impl rand_core::TryCryptoRng for HkdfRng {}

fn hex(c: &serde_json::Value, field: &str) -> Vec<u8> {
    hex::decode(c[field].as_str().unwrap()).unwrap()
}

#[test]
fn matches_go() {
    let v: serde_json::Value = serde_json::from_str(include_str!("ctidh1024.json")).unwrap();
    let cases = v["vectors"].as_array().unwrap();
    assert_eq!(cases.len(), 3);
    for c in cases {
        let name = c["name"].as_str().unwrap();
        for (label, field) in [
            ("nike_alice", "alice_private_key_hex"),
            ("nike_bob", "bob_private_key_hex"),
            ("nike_blinding_factor", "blinding_factor_hex"),
        ] {
            let key = PrivateKey::generate(&mut HkdfRng::new(label, name));
            assert_eq!(*key.to_bytes(), hex(c, field), "{name} {label}");
        }

        let alice = PrivateKey::from_bytes(&hex(c, "alice_private_key_hex")).unwrap();
        let bob = PrivateKey::from_bytes(&hex(c, "bob_private_key_hex")).unwrap();
        let factor = PrivateKey::from_bytes(&hex(c, "blinding_factor_hex")).unwrap();
        assert_eq!(*alice.to_bytes(), hex(c, "alice_private_key_hex"), "{name}");

        let alice_pk = alice.public_key();
        assert_eq!(
            alice_pk.to_bytes(),
            hex(c, "alice_public_key_hex"),
            "{name}"
        );
        assert_eq!(
            bob.public_key().to_bytes(),
            hex(c, "bob_public_key_hex"),
            "{name}"
        );

        let bob_pk = PublicKey::from_bytes(&hex(c, "bob_public_key_hex")).unwrap();
        assert_eq!(bob_pk, bob.public_key(), "{name}");
        let ss = alice.shared_secret(&bob_pk).unwrap();
        assert_eq!(*ss, hex(c, "shared_secret_hex"), "{name}");
        assert_eq!(*bob.shared_secret(&alice_pk).unwrap(), *ss, "{name}");

        assert_eq!(
            alice_pk.blind(&factor).unwrap().to_bytes(),
            hex(c, "blinded_alice_public_key_hex"),
            "{name}"
        );
    }
}
