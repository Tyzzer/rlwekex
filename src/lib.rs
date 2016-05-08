extern crate rand;
extern crate byteorder;
extern crate stepbymap;

mod util;
mod fft;
mod rlwe;
pub mod ct;
pub mod rlwe_key;

pub use fft::FFT;
pub use rlwe::RLWE_A;
use util::{
    u32_to_bytes, bytes_to_u32,
    u64_to_bytes, bytes_to_u64
};
use rlwe_key::{
    key_generate_keypair,
    kex_compute_key_alice,
    kex_compute_key_bob
};


/// ```
/// use rlwekex::RlweKex;
/// let alice = RlweKex::new();
/// let bob = RlweKex::new();
/// assert!(alice.private_export() != bob.private_export());
///
/// let (data, bob_secret) = bob.exchange(&alice.public_export());
/// let alice_secret = alice.exchange_from(&bob.public_export(), &data);
/// assert_eq!(alice_secret, bob_secret);
/// ```
pub struct RlweKex {
    sk: [u32; 1024],
    pub pk: [u32; 1024]
}

impl Default for RlweKex {
    fn default() -> RlweKex {
        let (sk, pk) = key_generate_keypair(&FFT::new(), &RLWE_A);
        RlweKex { sk: sk, pk: pk }
    }
}

impl RlweKex {
    pub fn new() -> RlweKex {
        RlweKex::default()
    }

    pub fn import(sk: &[u8], pk: &[u8]) -> RlweKex {
        RlweKex {
            sk: bytes_to_u32(sk),
            pk: bytes_to_u32(pk)
        }
    }

    pub fn private_export(&self) -> Vec<u8> {
        u32_to_bytes(&self.sk)
    }

    pub fn public_export(&self) -> Vec<u8> {
        u32_to_bytes(&self.pk)
    }

    pub fn exchange(&self, target: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let (data, secret) = kex_compute_key_bob(
            &FFT::new(),
            &bytes_to_u32(target),
            &self.sk
        );
        (u64_to_bytes(&data), u64_to_bytes(&secret))
    }

    pub fn exchange_from(&self, target: &[u8], data: &[u8]) -> Vec<u8> {
        u64_to_bytes(&kex_compute_key_alice(
            &FFT::new(),
            &bytes_to_u32(target),
            &self.sk,
            &bytes_to_u64(data)
        ))
    }
}
