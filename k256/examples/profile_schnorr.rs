use k256::{
    elliptic_curve::Generate,
    schnorr::{
        SigningKey,
        signature::{Signer, Verifier},
    },
};

fn main() {
    let sk = SigningKey::generate();
    let vk = sk.verifying_key().clone();
    let message = b"Schnorr benchmark message for performance testing";
    let sig = sk.sign(&message[..]);

    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("all");
    let iterations: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10000);

    match mode {
        "keygen" => {
            for _ in 0..iterations {
                std::hint::black_box(SigningKey::generate());
            }
        }
        "sign" => {
            for _ in 0..iterations {
                std::hint::black_box(sk.sign(&message[..]));
            }
        }
        "verify" => {
            for _ in 0..iterations {
                std::hint::black_box(vk.verify(&message[..], &sig).unwrap());
            }
        }
        "all" | _ => {
            for _ in 0..iterations {
                std::hint::black_box(SigningKey::generate());
            }
            for _ in 0..iterations {
                std::hint::black_box(sk.sign(&message[..]));
            }
            for _ in 0..iterations {
                std::hint::black_box(vk.verify(&message[..], &sig).unwrap());
            }
        }
    }
}
