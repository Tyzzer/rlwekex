use rand::{ Rand, OsRng, thread_rng, Rng };
use byteorder::{ BigEndian, ByteOrder, WriteBytesExt };

type Endian = BigEndian;


pub fn random<T: Rand>() -> T {
    match OsRng::new() {
        Ok(mut rng) => rng.gen(),
        _ => thread_rng().gen()
    }
}

pub fn u32_to_bytes(input: &[u32]) -> Vec<u8> {
    let mut output = Vec::new();
    for &b in input {
        output.write_u32::<Endian>(b).ok();
    }
    output
}

pub fn bytes_to_u32(input: &[u8]) -> [u32; 1024] {
    let mut output = [0; 1024];
    for i in 0..1024 {
        output[i] = Endian::read_u32(&input[(i * 4)..((i + 1) * 4)]);
    }
    output
}

pub fn u64_to_bytes(input: &[u64]) -> Vec<u8> {
    let mut output = Vec::new();
    for &b in input {
        output.write_u64::<Endian>(b).ok();
    }
    output
}

pub fn bytes_to_u64(input: &[u8]) -> [u64; 16] {
    let mut output = [0; 16];
    for i in 0..16 {
        output[i] = Endian::read_u64(&input[(i * 8)..((i + 1) * 8)]);
    }
    output
}
