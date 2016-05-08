#![feature(test)]

extern crate test;
extern crate rlwekex;

use test::Bencher;
use rlwekex::{ FFT, RLWE_A };
use rlwekex::rlwe_key::{
    key_generate_keypair,
    kex_compute_key_alice,
    kex_compute_key_bob
};


#[bench]
fn bench_key_generate_keypair(b: &mut Bencher) {
    let fft = FFT::new();
    b.iter(|| key_generate_keypair(&fft, &RLWE_A));
}

#[bench]
fn bench_kex_compute_key_alice(b: &mut Bencher) {
    let fft = FFT::new();
    let alice = key_generate_keypair(&fft, &RLWE_A);
    let bob = key_generate_keypair(&fft, &RLWE_A);
    let (c, _) = kex_compute_key_bob(&fft, &alice.1, &bob.0);
    b.iter(|| kex_compute_key_alice(&fft, &bob.1, &alice.0, &c));
}

#[bench]
fn bench_kex_compute_key_bob(b: &mut Bencher) {
    let fft = FFT::new();
    let alice = key_generate_keypair(&fft, &RLWE_A);
    let bob = key_generate_keypair(&fft, &RLWE_A);
    b.iter(|| kex_compute_key_bob(&fft, &alice.1, &bob.0));
}
