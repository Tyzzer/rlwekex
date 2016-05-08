use stepbymap::StepByMap;


pub struct FFT {
    x: [[u32; 64]; 64],
    y: [[u32; 64]; 64],
    z: [[u32; 64]; 64],
    t: [u32; 64]
}

impl Default for FFT {
    fn default() -> FFT {
        FFT {
            x: [[0; 64]; 64],
            y: [[0; 64]; 64],
            z: [[0; 64]; 64],
            t: [0; 64]
        }
    }
}

impl FFT {
    pub fn new() -> FFT { FFT::default() }

    pub fn mul(&self, x: &[u32], y: &[u32], z: &mut [u32]) {
        self.nussbaumer_fft(x, y, z)
    }

    pub fn add(x: &[u32], y: &[u32], z: &mut [u32]) {
        for i in 0..1024 {
            z[i] = mod_add(x[i], y[i]);
        }
    }

    fn nussbaumer_fft(&self, x: &[u32], y: &[u32], z: &mut [u32]) {
        let mut x1 = self.x;
        let mut y1 = self.y;
        let mut z1 = self.z;
        let mut t1 = self.t;

        for i in 0..32 {
            for j in 0..32 {
                x1[i][j] = x[32 * j + i];
                x1[i + 32][j] = x[32 * j + i];

                y1[i][j] = y[32 * j + i];
                y1[i + 32][j] = y[32 * j + i];
            }
        }

        for j in (0..5).rev() {
            for i in (0..).step_map(|n| n + 1).stop_map(|&n| n >= (1 << (5 - j))) {
                let ssr = reverse(i);
                for t in (0..).step_map(|n| n + 1).stop_map(|&n| n >= (1 << j)) {
                    let s = i << (j + 1);
                    let sr = ((ssr >> (32 - 5 + j)) << j) as usize;

                    let ii = (s + t) as usize;
                    let ll = (s + t + (1 << j)) as usize;

                    for a in sr..32 {
                        t1[a] = x1[ll][a - sr];
                    }
                    for a in 0..sr {
                        t1[a] = neg(x1[ll][32 + a - sr]);
                    }
                    for a in 0..32 {
                        x1[ll][a] = mod_sub(x1[ii][a], t1[a]);
                        x1[ii][a] = mod_add(x1[ii][a], t1[a]);
                    }

                    for a in sr..32 {
                        t1[a] = y1[ll][a - sr];
                    }
                    for a in 0..sr {
                        t1[a] = neg(y1[ll][32 + a - sr]);
                    }
                    for a in 0..32 {
                        y1[ll][a] = mod_sub(y1[ii][a], t1[a]);
                        y1[ii][a] = mod_add(y1[ii][a], t1[a]);
                    }
                }
            }
        }

        for i in 0..(2 * 32) {
            naive(&x1[i], &y1[i], &mut z1[i], 32);
        }

        for j in 0..6 {
            for i in (0..).step_map(|n| n + 1).stop_map(|&n| n >= (1 << (5 - j))) {
                let ssr = reverse(i);
                for t in (0..).step_map(|n| n + 1).stop_map(|&n| n >= (1 << j)) {
                    let s = i << (j + 1);
                    let sr = (ssr.wrapping_shr(32 - 5 + j) << j) as usize;

                    let aa = (s + t) as usize;
                    let bb = (s + t + (1 << j)) as usize;

                    for a in 0..32 {
                        t1[a] = mod_sub(z1[aa][a], z1[bb][a]);
                        t1[a] = mod_div2(t1[a]);
                        z1[aa][a] = mod_add(z1[aa][a], z1[bb][a]);
                        z1[aa][a] = mod_div2(z1[aa][a]);
                    }

                    for a in 0..(32 - sr) {
                        z1[bb][a] = t1[a + sr];
                    }
                    for a in (32 - sr)..32 {
                        z1[bb][a] = neg(t1[a - (32 - sr)]);
                    }
                }
            }
        }

        for i in 0..32 {
            z[i] = mod_sub(z1[i][0], z1[32 + i][32 - 1]);
            for j in 1..32 {
                z[32 * j + i] = mod_add(z1[i][j], z1[32 + i][j - 1]);
            }
        }
    }
}


fn naive(x: &[u32], y: &[u32], z: &mut [u32], n: usize) {
    for i in 0..n {
        let mut a = mod_mul(x[0], y[i]);
        let mut b = 0;

        for j in 1..n {
            if j <= i {
                a = mod_mul_add(x[j], y[i-j], a);
            } else {
                b = mod_mul_add(x[j], y[n-(j-i)], b);
            }
        }

        z[i] = mod_sub(a, b);
    }
}

fn reverse(mut x: u32) -> u32 {
	x = ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1);
	x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
	x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
	x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
	(x >> 16) | (x << 16)
}

fn mod_add(a: u32, b: u32) -> u32 {
    let t = a.wrapping_add(b);
    t.wrapping_add(if t < a { 1 } else { 0 })
}

fn mod_sub(a: u32, b: u32) -> u32 {
    a.wrapping_sub(b).wrapping_sub(if b > a { 1 } else { 0 })
}

fn mod_mul(a: u32, b: u32) -> u32 {
    let t = a as u64 * b as u64;
    mod_add(t as u32, (t >> 32) as u32)
}

fn mod_mul_add(a: u32, b: u32, c: u32) -> u32 {
    let t = a as u64 * b as u64 + c as u64;
    mod_add(t as u32, (t >> 32) as u32)
}

fn div2(a: u32) -> u32 {
    ((a as u64 + (0u32.wrapping_sub(a & 1) & 0xffffffff) as u64) >> 1) as u32
}

fn normalize(a: u32) -> u32 {
    a.wrapping_add(if a == 0xffffffff { 1 } else { 0 })
}

fn mod_div2(a: u32) -> u32 {
    div2(normalize(a))
}

fn neg(a: u32) -> u32 {
    normalize(0xffffffff - a)
}
