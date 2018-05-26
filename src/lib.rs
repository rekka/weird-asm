pub fn precomp_damped_jacobi(
    w: &mut [f64],
    u: &[f64],
    b: &[f64],
    a4_rcp: &[f64],
    dim: (usize, usize),
    omega: f64,
) {
    let (ni, nj) = dim;
    let (si, sj) = (nj, 1);
    for i in 1..ni - 1 {
        // periodic in j
        let s = i * nj;
        w[s] = (1. - omega) * u[s]
            + omega * (b[s] + u[s - si] + u[s + si] + u[s + nj - 1] + u[s + 1]) * a4_rcp[s];
        {
            // help rustc elide bound checks
            let len = nj - 2;
            let uc = &u[s + 1..][..len];
            let ul = &u[s..][..len];
            let ur = &u[s + 2..][..len];
            let ub = &u[s + 1 - nj..][..len];
            let ut = &u[s + 1 + nj..][..len];
            let bc = &b[s + 1..][..len];
            let a4_rcpc = &a4_rcp[s + 1..][..len];
            let wc = &mut w[s + 1..][..len];
            for p in 0..len {
                wc[p] = (1. - omega) * uc[p]
                    + omega * (bc[p] + ul[p] + ur[p] + ut[p] + ub[p]) * a4_rcpc[p];
            }
        }
        let s = i * nj + nj - 1;
        w[s] = (1. - omega) * u[s]
            + omega * (b[s] + u[s - si] + u[s + si] + u[s - 1] + u[s - (nj - 1)]) * a4_rcp[s];
    }
    // Neumann at i = 0
    for j in 0..nj {
        let s = j;
        let sjp = if j < nj - 1 {
            s + sj
        } else {
            s - (nj - 1) * sj
        };
        let sjm = if j > 0 { s - sj } else { s + (nj - 1) * sj };
        w[s] =
            (1. - omega) * u[s] + omega * (b[s] + 2. * u[s + si] + u[sjm] + u[sjp]) * a4_rcp[s];
    }
}

