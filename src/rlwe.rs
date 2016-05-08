use rand::random;
use ::ct::{ select, cmplt, eq, ge, le };


include!("rlwe_args.rs");


fn get_bit(a: &[u64], x: usize) -> u64 {
    (a[x / 64] >> (x as u64 % 64)) & 1
}

pub fn single_sample(input: &[u64]) -> u32 {
    (0..52).fold(0, |sum, next| select(
        sum,
        next as u64 + 1,
        cmplt(input, &RLWE_TABLE[next])
    )) as u32
}

pub fn dbl(input: u32, mut e: i32) -> u64 {
    e = ((e >> 1) & 1) - (e & 1);
    ((input as u64) << 1) - e as u64
}

pub fn sample(s: &mut [u32]) {
    for i in 0..16 {
        let mut r: u64 = random();
        for j in 0..64 {
            let m = r & 1;
            r >>= 1;
            let t = single_sample(&[random(), random(), random()]);
            s[i * 64 + j] = select((::std::u32::MAX - t) as u64, t as u64, eq(m, 0)) as u32;
        }
    }
}

pub fn round2(input: &[u32]) -> [u64; 16] {
    let mut output = [0; 16];
    for i in 0..1024 {
        let b = ge(input[i] as u64, 1073741824) & le(input[i] as u64, 3221225471);
        output[i / 64] |= b << (i / 64) as u64;
    }
    output
}

pub fn crossround2(input: &[u32]) -> [u64; 16] {
    let mut output = [0; 16];

    for i in 0..64 {
        let mut e: u32 = random();
        for j in 0..16 {
            let dd = dbl(input[i * 16 + j], e as i32);
            e >>= 2;
            let b = (ge(dd, 2147483648) & le(dd, 4294967295))
                | (ge(dd, 6442450942) & le(dd, 8589934590));
            output[(i * 16 + j) / 64] |= b << ((i * 16 +j) % 64);
        }
    }

    output
}

pub fn rec(w: &[u32], b: &[u64]) -> [u64; 16] {
    let mut output = [0; 16];

    for i in 0..1024 {
        let coswi = (w[i] as u64) << 1;
        let bb = (eq(get_bit(b, i), 0) & ge(coswi, 3221225472) & le(coswi, 7516192766))
            | (eq(get_bit(b, i), 1) & ge(coswi, 1073741824) & le(coswi, 5368709118));
        output[i / 64] = bb << (i % 64);
    }

    output
}
