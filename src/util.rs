use rand::{ Rand, OsRng, thread_rng, Rng };
use byteorder::{ BigEndian, WriteBytesExt, ReadBytesExt };

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

pub fn bytes_to_u32(mut input: &[u8]) -> [u32; 1024] {
    let mut output = [0; 1024];
    let mut data = Vec::with_capacity(1024);
    loop {
        data.push(match input.read_u32::<Endian>() {
            Ok(v) => v,
            _ => break
        });
    }
    output.clone_from_slice(&data);
    output
}

pub fn u64_to_bytes(input: &[u64]) -> Vec<u8> {
    let mut output = Vec::new();
    for &b in input {
        output.write_u64::<Endian>(b).ok();
    }
    output
}

pub fn bytes_to_u64(mut input: &[u8]) -> [u64; 16] {
    let mut output = [0; 16];
    let mut data = Vec::with_capacity(16);
    loop {
        data.push(match input.read_u64::<Endian>() {
            Ok(v) => v,
            _ => break
        });
    }
    output.clone_from_slice(&data);
    output
}
