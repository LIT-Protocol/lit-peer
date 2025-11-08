use elliptic_curve::PrimeField;

pub fn lagrange<F: PrimeField>(xi: F, participants: &[F]) -> F {
    let mut num = F::ONE;
    let mut den = F::ONE;
    for &xj in participants {
        if xi == xj {
            continue;
        }
        num *= xj;
        den *= xj - xi;
    }
    num * den.invert().expect("Denominator should not be zero")
}
