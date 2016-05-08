extern crate rlwekex;

use rlwekex::FFT;
use rlwekex::RLWE_A;
use rlwekex::rlwe_key::{
    key_generate_keypair,
    kex_compute_key_alice,
    kex_compute_key_bob
};


#[test]
fn test_rlwe_key() {
    let alice = key_generate_keypair(&FFT::new(), &RLWE_A);
    let bob = key_generate_keypair(&FFT::new(), &RLWE_A);

    let (c, bob_secret) = kex_compute_key_bob(
        &FFT::new(),
        &alice.1,
        &bob.0
    );
    let alice_secret = kex_compute_key_alice(
        &FFT::new(),
        &bob.1,
        &alice.0,
        &c
    );

    assert_eq!(alice_secret, bob_secret);
}
