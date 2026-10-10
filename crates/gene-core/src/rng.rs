//! RNG-1 (Philox4x32-10 with BLAKE3-derived stream keys) and FluxRng-1. SPEC §14.

const PM0: u32 = 0xD251_1F53;
const PM1: u32 = 0xCD9E_8D57;
const PW0: u32 = 0x9E37_79B9;
const PW1: u32 = 0xBB67_AE85;

#[inline]
pub fn philox(ctr: [u32; 4], key: [u32; 2]) -> [u32; 4] {
    let mut c = ctr;
    let mut k = key;
    for i in 0..10 {
        if i > 0 {
            k[0] = k[0].wrapping_add(PW0);
            k[1] = k[1].wrapping_add(PW1);
        }
        let p0 = (PM0 as u64) * (c[0] as u64);
        let p1 = (PM1 as u64) * (c[2] as u64);
        c = [
            ((p1 >> 32) as u32) ^ c[1] ^ k[0],
            p1 as u32,
            ((p0 >> 32) as u32) ^ c[3] ^ k[1],
            p0 as u32,
        ];
    }
    c
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Domain {
    WorldGen = 1,
    Controller = 2,
    Mutation = 3,
    Resolve = 4,
    Environment = 5,
    IdTie = 6,
    Observer = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FluxDomain {
    EdgeFlux = 8,
    PoreFlux = 9,
    ReactionRound = 10,
}

pub fn seed_bytes(seed: u64) -> [u8; 16] {
    let mut s = [0u8; 16];
    s[..8].copy_from_slice(&seed.to_le_bytes());
    s
}

fn key_from_hash(h: &blake3::Hash) -> [u32; 2] {
    let b = h.as_bytes();
    [
        u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
    ]
}

pub fn stream_key(seed: u64, domain: Domain, object: u64, ordinal: u64) -> [u32; 2] {
    let mut h = blake3::Hasher::new();
    h.update(b"civ-rng-v1");
    h.update(&[0u8]);
    h.update(&seed_bytes(seed));
    h.update(&(domain as u32).to_le_bytes());
    h.update(&object.to_le_bytes());
    h.update(&ordinal.to_le_bytes());
    key_from_hash(&h.finalize())
}

pub fn flux_key(seed: u64, domain: FluxDomain, selector: u32) -> [u32; 2] {
    let mut h = blake3::Hasher::new();
    h.update(b"civ-flux-v1");
    h.update(&[0u8]);
    h.update(&seed_bytes(seed));
    h.update(&(domain as u32).to_le_bytes());
    h.update(&selector.to_le_bytes());
    key_from_hash(&h.finalize())
}

/// FluxRng-1 draw: r = lane0 & (d - 1), d a power of two.
#[inline]
pub fn flux_r(key: [u32; 2], object: u64, tick: u64, d: u32) -> u32 {
    debug_assert!(d.is_power_of_two());
    let out = philox(
        [object as u32, (object >> 32) as u32, tick as u32, (tick >> 32) as u32],
        key,
    );
    out[0] & (d - 1)
}

/// Stochastic rounding SR(N, D) = floor(N/D) + [r < N mod D]. Draws only when needed.
#[inline]
pub fn sr(n: u64, d: u32, key: [u32; 2], object: u64, tick: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let q = n / d as u64;
    let rem = (n % d as u64) as u32;
    if rem == 0 {
        return q;
    }
    q + (flux_r(key, object, tick, d) < rem) as u64
}

/// A counter-based draw sequence: counter = (tick_lo, tick_hi, block_lo, block_hi).
#[derive(Clone, Debug)]
pub struct Stream {
    key: [u32; 2],
    tick: u64,
    d: u64,
    block: [u32; 4],
    block_idx: u64,
}

impl Stream {
    pub fn new(key: [u32; 2], tick: u64) -> Self {
        Self::at(key, tick, 0)
    }

    pub fn at(key: [u32; 2], tick: u64, d: u64) -> Self {
        Stream { key, tick, d, block: [0; 4], block_idx: u64::MAX }
    }

    pub fn draws_used(&self) -> u64 {
        self.d
    }

    pub fn next_u32(&mut self) -> u32 {
        let b = self.d / 4;
        if b != self.block_idx {
            self.block = philox(
                [self.tick as u32, (self.tick >> 32) as u32, b as u32, (b >> 32) as u32],
                self.key,
            );
            self.block_idx = b;
        }
        let v = self.block[(self.d % 4) as usize];
        self.d += 1;
        v
    }

    pub fn next_u16(&mut self) -> u16 {
        self.next_u32() as u16
    }

    /// Uniform in 0..n (n > 0) by rejection sampling.
    pub fn below(&mut self, n: u32) -> u32 {
        assert!(n > 0);
        if n.is_power_of_two() {
            return self.next_u32() & (n - 1);
        }
        let zone = ((1u64 << 32) / n as u64) * n as u64;
        loop {
            let x = self.next_u32() as u64;
            if x < zone {
                return (x % n as u64) as u32;
            }
        }
    }

    /// Inclusive range lo..=hi.
    pub fn range(&mut self, lo: u32, hi: u32) -> u32 {
        lo + self.below(hi - lo + 1)
    }
}

/// Fisher-Yates permutation of 0..n, i from n-1 down to 1, j uniform in 0..=i.
pub fn permutation(stream: &mut Stream, n: usize) -> Vec<usize> {
    let mut p: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        let j = stream.below(i as u32 + 1) as usize;
        p.swap(i, j);
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn philox_known_answer() {
        assert_eq!(philox([0; 4], [0; 2]), [0x6627e8d5, 0xe169c58d, 0xbc57ac4c, 0x9b00dbd8]);
    }

    #[test]
    fn philox_random123_vectors() {
        // Random123 kat_vectors: philox4x32_10 all-ones and pi inputs.
        assert_eq!(
            philox([u32::MAX; 4], [u32::MAX; 2]),
            [0x408f276d, 0x41c83b0e, 0xa20bc7c6, 0x6d5451fd]
        );
        assert_eq!(
            philox([0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344], [0xa4093822, 0x299f31d0]),
            [0xd16cfe09, 0x94fdcceb, 0x5001e420, 0x24126ea1]
        );
    }

    #[test]
    fn below_is_in_range_and_deterministic() {
        let k = stream_key(1, Domain::Mutation, 5, 7);
        let mut a = Stream::new(k, 3);
        let mut b = Stream::new(k, 3);
        for n in 1..200u32 {
            let x = a.below(n);
            assert!(x < n);
            assert_eq!(x, b.below(n));
        }
    }

    #[test]
    fn sr_conserves_expectation() {
        let key = flux_key(9, FluxDomain::EdgeFlux, 1);
        let mut total = 0u64;
        for t in 0..8192u64 {
            total += sr(256, 2048, key, 42, t); // expectation 1/8
        }
        assert!((900..1150).contains(&total), "total {total}");
        assert_eq!(sr(4096, 2048, key, 1, 1), 2);
        assert_eq!(sr(0, 2048, key, 1, 1), 0);
    }
}
