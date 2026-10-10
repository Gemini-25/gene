//! Species table, elements, reactions. SPEC §4, §5.3.

pub const NSPEC: usize = 10;
pub const W: usize = 0;
pub const C: usize = 1;
pub const O: usize = 2;
pub const G: usize = 3;
pub const H: usize = 4;
pub const X: usize = 5;
pub const M: usize = 6;
pub const S: usize = 7;
pub const V: usize = 8;
pub const I: usize = 9;

pub const SYMBOL: [char; NSPEC] = ['W', 'C', 'O', 'G', 'H', 'X', 'M', 'S', 'V', 'I'];

/// Mass per unit.
pub const MASS: [u64; NSPEC] = [18, 44, 32, 30, 2, 72, 56, 60, 18, 18];
/// Effective volume per unit.
pub const VOL: [u32; NSPEC] = [1, 4, 4, 1, 4, 1, 1, 1, 4, 1];
/// Heat capacity per unit.
pub const CAP: [u64; NSPEC] = [3, 3, 2, 4, 2, 2, 1, 3, 3, 3];
/// Formation energy per unit (model value).
pub const ENTH: [i64; NSPEC] = [-3000, -4000, 0, -2000, 0, -2800, 0, -9000, -2560, -3060];

/// Element order C, H, O, Fe, Si.
pub const NELEM: usize = 5;
pub const ELEM_MASS: [u64; NELEM] = [12, 1, 16, 56, 28];
pub const COMPOSITION: [[u8; NELEM]; NSPEC] = [
    [0, 2, 1, 0, 0], // W  H2O
    [1, 0, 2, 0, 0], // C  CO2
    [0, 0, 2, 0, 0], // O  O2
    [1, 2, 1, 0, 0], // G  CH2O
    [0, 2, 0, 0, 0], // H  H2
    [0, 0, 1, 1, 0], // X  FeO
    [0, 0, 0, 1, 0], // M  Fe
    [0, 0, 2, 0, 1], // S  SiO2
    [0, 2, 1, 0, 0], // V  H2O vapour
    [0, 2, 1, 0, 0], // I  H2O ice
];

/// Fixed reaction G+O -> C+W energy split for respiration: available part.
pub const RESP_AVAIL: u32 = 1666;
pub const RESP_HEAT: u64 = 3334;
/// Photosynthesis consumes this much photo credit per extent.
pub const PHOTO_COST: u32 = 10_000;
pub const PHOTO_HEAT: u64 = 5000;
pub const PHOTO_CREDIT_CAP: u32 = 49_999;

pub fn volume(stock: &[u16; NSPEC]) -> u32 {
    let mut v = 0;
    for i in 0..NSPEC {
        v += stock[i] as u32 * VOL[i];
    }
    v
}

pub fn heat_cap(stock: &[u64; NSPEC]) -> u64 {
    (0..NSPEC).map(|i| stock[i] * CAP[i]).sum()
}

pub fn mass(stock: &[u64; NSPEC]) -> u64 {
    (0..NSPEC).map(|i| stock[i] * MASS[i]).sum()
}

pub fn enthalpy(stock: &[u64; NSPEC]) -> i128 {
    (0..NSPEC).map(|i| stock[i] as i128 * ENTH[i] as i128).sum()
}

pub fn add_atoms(acc: &mut [u128; NELEM], stock: &[u64; NSPEC]) {
    for i in 0..NSPEC {
        for e in 0..NELEM {
            acc[e] += stock[i] as u128 * COMPOSITION[i][e] as u128;
        }
    }
}

/// Generic loose-site reaction (Rx2..Rx8). Coefficients are per extent.
pub struct Reaction {
    pub id: u8,
    pub inputs: &'static [(usize, u16)],
    pub outputs: &'static [(usize, u16)],
    /// Heat released into U per extent (negative absorbs).
    pub heat: i64,
    /// Rates per temperature bin (<270, 270..329, 330..499, 500..699, 700..899, >=900).
    pub rates: [u32; 6],
    pub min_theta: u64,
    /// Requires loose O <= 1 (reducing condition).
    pub reducing: bool,
}

pub const RX2: Reaction = Reaction {
    id: 2,
    inputs: &[(G, 1), (O, 1)],
    outputs: &[(C, 1), (W, 1)],
    heat: 5000,
    rates: [0, 1, 4, 256, 1024, 4096],
    min_theta: 0,
    reducing: false,
};
pub const RX3: Reaction = Reaction {
    id: 3,
    inputs: &[(H, 2), (O, 1)],
    outputs: &[(V, 2)],
    heat: 5120,
    rates: [0, 0, 0, 4096, 4096, 4096],
    min_theta: 500,
    reducing: false,
};
pub const RX4: Reaction = Reaction {
    id: 4,
    inputs: &[(X, 2), (G, 1)],
    outputs: &[(M, 2), (C, 1), (V, 1)],
    heat: -1040,
    rates: [0, 0, 0, 0, 0, 4096],
    min_theta: 900,
    reducing: true,
};
pub const RX5: Reaction = Reaction {
    id: 5,
    inputs: &[(X, 1), (H, 1)],
    outputs: &[(M, 1), (V, 1)],
    heat: -240,
    rates: [0, 0, 0, 0, 4096, 4096],
    min_theta: 700,
    reducing: true,
};
pub const RX6: Reaction = Reaction {
    id: 6,
    inputs: &[(M, 2), (O, 1)],
    outputs: &[(X, 2)],
    heat: 5600,
    rates: [0, 1, 4, 32, 128, 512],
    min_theta: 0,
    reducing: false,
};

pub const CREDIT_REACTIONS: [&Reaction; 5] = [&RX2, &RX3, &RX4, &RX5, &RX6];

/// Rate of a credit reaction at temperature theta. With faster decomposition (ruleset
/// `gene-m0-2` and later, SPEC §20h) Rx2 runs at least 16 in the living bins 270..499 K.
#[inline]
pub fn rx_rate(rx: &Reaction, theta: u64, decomp: bool) -> u64 {
    let bin = temp_bin(theta);
    let r = rx.rates[bin] as u64;
    if decomp && rx.id == 2 && (bin == 1 || bin == 2) {
        r.max(DECOMP_RATE)
    } else {
        r
    }
}

/// Rx2 rate floor at living temperatures under faster decomposition.
pub const DECOMP_RATE: u64 = 16;

pub fn temp_bin(theta: u64) -> usize {
    match theta {
        0..=269 => 0,
        270..=329 => 1,
        330..=499 => 2,
        500..=699 => 3,
        700..=899 => 4,
        _ => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(inputs: &[(usize, u16)], outputs: &[(usize, u16)], dh: i64) {
        let mut atoms_in = [0i64; NELEM];
        let mut atoms_out = [0i64; NELEM];
        let (mut h_in, mut h_out, mut c_in, mut c_out, mut m_in, mut m_out) = (0, 0, 0, 0, 0, 0);
        for &(s, n) in inputs {
            for e in 0..NELEM {
                atoms_in[e] += COMPOSITION[s][e] as i64 * n as i64;
            }
            h_in += ENTH[s] * n as i64;
            c_in += CAP[s] * n as u64;
            m_in += MASS[s] * n as u64;
        }
        for &(s, n) in outputs {
            for e in 0..NELEM {
                atoms_out[e] += COMPOSITION[s][e] as i64 * n as i64;
            }
            h_out += ENTH[s] * n as i64;
            c_out += CAP[s] * n as u64;
            m_out += MASS[s] * n as u64;
        }
        assert_eq!(atoms_in, atoms_out);
        assert_eq!(h_out - h_in, dh);
        assert_eq!(c_in, c_out, "heat capacity must be invariant");
        assert_eq!(m_in, m_out, "mass must be invariant");
    }

    #[test]
    fn species_masses_match_elements() {
        for s in 0..NSPEC {
            let m: u64 = (0..NELEM).map(|e| COMPOSITION[s][e] as u64 * ELEM_MASS[e]).sum();
            assert_eq!(m, MASS[s], "species {}", SYMBOL[s]);
        }
    }

    #[test]
    fn reactions_conserve() {
        check(&[(C, 1), (W, 1)], &[(G, 1), (O, 1)], 5000); // Rx0
        check(&[(G, 1), (O, 1)], &[(C, 1), (W, 1)], -5000); // Rx1
        for r in CREDIT_REACTIONS {
            check(r.inputs, r.outputs, -r.heat);
        }
        check(&[(W, 1)], &[(V, 1)], 440); // Rx7
        check(&[(W, 1)], &[(I, 1)], -60); // Rx8
        assert_eq!(RESP_AVAIL as u64 + RESP_HEAT, 5000);
    }
}
