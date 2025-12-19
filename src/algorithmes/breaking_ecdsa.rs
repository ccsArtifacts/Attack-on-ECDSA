use rug::Integer;

/// Build a lattice-like matrix for an educational ECDSA attack scenario.
///
/// ⚠️ Educational note:
/// This function does **not** run LLL; it only prepares a matrix that could be
/// passed to an external lattice reduction tool.
pub fn break_ecdsa(samples: &[(u64, Integer, Integer, Integer)], msb: usize) -> Vec<Vec<Integer>> {
    let q = Integer::from(1) << 256;

    let mut a_i = Vec::with_capacity(samples.len());
    let mut t_i = Vec::with_capacity(samples.len());

    let _msb_bits = msb;

    for (k_msb, h, r, s) in samples.iter() {
        let s_inv = s.clone().invert(&q).expect("s is not invertible modulo q");

        let mut a = Integer::from(*k_msb);
        let sh = Integer::from(&s_inv * h);
        a -= sh;
        a %= &q;
        if a < 0 {
            a += &q;
        }

        let mut t = Integer::from(&s_inv * r);
        t %= &q;
        if t < 0 {
            t += &q;
        }

        a_i.push(a);
        t_i.push(t);
    }

    let m = samples.len();
    let n = m + 1;

    let mut mat = vec![vec![Integer::from(0); n]; n];

    // ✅ correction Clippy : boucle idiomatique
    for (i, row) in mat.iter_mut().enumerate() {
        row[i] = Integer::from(1);
    }

    for i in 0..m {
        mat[i][m] = t_i[i].clone();
        mat[m][i] = a_i[i].clone();
    }

    mat
}
