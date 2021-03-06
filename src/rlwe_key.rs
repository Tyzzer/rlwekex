//! Raw Function

use ::FFT;
use ::rlwe::{ sample, rec, crossround2, round2 };


pub fn key_generate_keypair(fft: &FFT, a: &[u32]) -> ([u32; 1024], [u32; 1024]) {
    let (mut s, mut b) = ([0; 1024], [0; 1024]);
    let mut e = [0; 1024];

    sample(&mut s);
    sample(&mut e);

    fft.mul(&a, &s, &mut b);
    let mut x = [0; 1024];
    x.clone_from_slice(&b);
    FFT::add(&x, &e, &mut b);

    (s, b)
}

pub fn kex_compute_key_alice(fft: &FFT, b: &[u32], s: &[u32], c: &[u64]) -> [u64; 16] {
    let mut w = [0; 1024];
    fft.mul(&b, &s, &mut w);
    rec(&w, &c)
}

pub fn kex_compute_key_bob(fft: &FFT, b: &[u32], s: &[u32]) -> ([u64; 16], [u64; 16]) {
    let mut v = [0; 1024];
    let mut eprimeprime = [0; 1024];

    sample(&mut eprimeprime);

    fft.mul(&b, &s, &mut v);
    let mut x = [0; 1024];
    x.clone_from_slice(&v);
    FFT::add(&x, &eprimeprime, &mut v);

    (crossround2(&v), round2(&v))
}
