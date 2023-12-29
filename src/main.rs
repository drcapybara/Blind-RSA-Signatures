use num_bigint::BigUint;
use rand::{rngs::OsRng, Rng};
use rsa::{PaddingScheme, PublicKeyParts, RsaPrivateKey, RsaPublicKey};

fn main() {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // Message
    let m = BigUint::from_bytes_be(message);
    let message = b"secret message";
    
    // Alice's public key
    let e = dbg!(BigUint::from_bytes_be(public_key.e().to_bytes_be().as_slice()));
    let n = dbg!(BigUint::from_bytes_be(public_key.n().to_bytes_be().as_slice()));

    // Sign the message
    let padding = PaddingScheme::new_pkcs1v15_sign(None);
    let signature = private_key
        .sign(padding, &message[..])
        .expect("failed to sign message");
    let s = dbg!(BigUint::from_bytes_be(&signature));

    // Number of iterations 'k'
    let k = 20;

    for i in 0..k {
        // Prover picks a random r_i
        let r_i = dbg!(BigUint::from(rng.gen::<u32>()));
        let c_i = dbg!(r_i.modpow(&e, &n));

        // Challenger picks a random bit b_i
        let b_i = dbg!(rng.gen::<bool>());

        // Prover's response
        let z_i = dbg!(if b_i {
            // Prover sends c_z_i = (s * r_i) (mod n)
            (s.clone() * r_i.clone()) % &n
        } else {
            // Prover sends r_i
            r_i.clone()
        });

        // Verifier's verification
        if b_i {
            let c_z_i = dbg!(z_i.modpow(&e, &n));
            assert_eq!(
                c_z_i,
                dbg!(m.clone() * c_i) % &n),
                "Verification failed at iteration {}",
                i
            );
        } else {
            assert_eq!(
                dbg!(z_i.modpow(&e, &n)),
                c_i,
                "Verification failed at iteration {}",
                i
            );
        }
    }

    println!("Protocol complete!");
}
