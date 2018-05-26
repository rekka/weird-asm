extern crate weird_asm;

use weird_asm::precomp_damped_jacobi;


fn main() {
    let n = 1024;

    let u = vec![0.; n * n];
    let b = vec![0.; n * n];
    let a4_rcp = vec![0.; n * n];
    let mut w = vec![0.; n * n];

    precomp_damped_jacobi(&mut w, &u, &b, &a4_rcp, (n, n), 2. / 3.);
}
