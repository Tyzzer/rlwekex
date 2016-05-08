#![feature(test)]

extern crate test;
extern crate rlwekex;

use test::Bencher;
use rlwekex::{ FFT, RLWE_A };


#[bench]
fn bench_fft_mul(b: &mut Bencher) {
    let fft = FFT::new();
    b.iter(|| fft.mul(&RLWE_A, &RLWE_A, &mut [0; 1024]));
}

#[bench]
fn bench_fft_add(b: &mut Bencher) {
    b.iter(|| FFT::add(&RLWE_A, &RLWE_A, &mut [0; 1024]));
}
