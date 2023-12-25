use rsa::{PaddingScheme, RsaPrivateKey, RsaPublicKey, PublicKeyParts};
use rand::{rngs::OsRng, Rng};
use num_bigint::BigUint;

fn main() {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    let message = b"secret message";

    // Alices public key
    let e = BigUint::from_bytes_be(public_key.e().to_bytes_be().as_slice());
    let n = BigUint::from_bytes_be(public_key.n().to_bytes_be().as_slice());

    // Sign the message
    let padding = PaddingScheme::new_pkcs1v15_sign(None);
    let signature = private_key.sign(padding, &message[..]).expect("failed to sign message");
    let s = BigUint::from_bytes_be(&signature);

    // Encrypt the signature: v = s^e (mod n)
    let v = s.modpow(&e, &n);

    // Number of iterations 'k'
    let k = 20;

    for i in 0..k {
        // Prover picks a random r_i
        let r_i = BigUint::from(rng.gen::<u64>());
        let c_i = r_i.modpow(&e, &n);

        // Challenger picks a random bit b_i
        let b_i = rng.gen::<bool>();

        // Prover's response
        let z_i = if b_i {
            // Prover sends c_z_i = (s * r_i)^e (mod n)
            s.modpow(&BigUint::from(1_u64), &n) * r_i.modpow(&BigUint::from(1_u64), &n)
        } else {
            // Prover sends r_i
            r_i
        };

        // Verifier's verification
        if b_i {
            let c_z_i = z_i.modpow(&e, &n);
            assert_eq!(c_z_i, v.clone() * c_i % &n, "Verification failed at iteration {}", i);
        } else {
            assert_eq!(z_i.modpow(&e, &n), c_i, "Verification failed at iteration {}", i);
        }
    }

    println!("Protocol complete!");
}