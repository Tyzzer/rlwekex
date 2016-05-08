extern crate rlwekex;

use rlwekex::fft::FFT;
use rlwekex::rlwe::RLWE_A;
use rlwekex::rlwe_key::{
    key_generate_keypair,
    kex_compute_key_alice,
    kex_compute_key_bob
};


#[test]
fn test_rlwe_key() {
    let mut fft = FFT::new();
    let alice = key_generate_keypair(&mut fft, &RLWE_A);
    let bob = key_generate_keypair(&mut fft, &RLWE_A);

    let (c, bob_secret) = kex_compute_key_bob(
        &mut fft,
        &alice.1,
        &bob.0
    );
    let alice_secret = kex_compute_key_alice(
        &mut fft,
        &bob.1,
        &alice.0,
        &c
    );

    assert_eq!(alice_secret, bob_secret);
}
