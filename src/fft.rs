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

    pub fn mul(&mut self, x: &[u32], y: &[u32], z: &mut [u32]) {
        self.nussbaumer_fft(x, y, z)
    }

    pub fn add(x: &[u32], y: &[u32], z: &mut [u32]) {
        for i in 0..1024 {
            z[i] = mod_add(x[i], y[i]);
        }
    }

    fn nussbaumer_fft(&mut self, x: &[u32], y: &[u32], z: &mut [u32]) {
        for i in 0..32 {
            for j in 0..32 {
                self.x[i][j] = x[32 * j + i];
                self.x[i + 32][j] = x[32 * j + i];

                self.y[i][j] = y[32 * j + i];
                self.y[i + 32][j] = y[32 * j + i];
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
                        self.t[a] = self.x[ll][a - sr];
                    }
                    for a in 0..sr {
                        self.t[a] = neg(self.x[ll][32 + a - sr]);
                    }
                    for a in 0..32 {
                        self.x[ll][a] = mod_sub(self.x[ii][a], self.t[a]);
                        self.x[ii][a] = mod_add(self.x[ii][a], self.t[a]);
                    }

                    for a in sr..32 {
                        self.t[a] = self.y[ll][a - sr];
                    }
                    for a in 0..sr {
                        self.t[a] = neg(self.y[ll][32 + a - sr]);
                    }
                    for a in 0..32 {
                        self.y[ll][a] = mod_sub(self.y[ii][a], self.t[a]);
                        self.y[ii][a] = mod_add(self.y[ii][a], self.t[a]);
                    }
                }
            }
        }

        for i in 0..(2 * 32) {
            naive(&self.x[i], &self.y[i], &mut self.z[i], 32);
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
                        self.t[a] = mod_sub(self.z[aa][a], self.z[bb][a]);
                        self.t[a] = mod_div2(self.t[a]);
                        self.z[aa][a] = mod_add(self.z[aa][a], self.z[bb][a]);
                        self.z[aa][a] = mod_div2(self.z[aa][a]);
                    }

                    for a in 0..(32 - sr) {
                        self.z[bb][a] = self.t[a + sr];
                    }
                    for a in (32 - sr)..32 {
                        self.z[bb][a] = neg(self.t[a - (32 - sr)]);
                    }
                }
            }
        }

        for i in 0..32 {
            z[i] = mod_sub(self.z[i][0], self.z[32 + i][32 - 1]);
            for j in 1..32 {
                z[32 * j + i] = mod_add(self.z[i][j], self.z[32 + i][j - 1]);
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
