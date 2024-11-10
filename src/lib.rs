mod errors;
mod okamoto;

pub use crate::errors::*;
pub use crate::okamoto::*;

#[cfg(test)]
mod tests {
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar;

    use super::*;

    #[test]
    fn dl_repr() {
        let generator_u = RISTRETTO_BASEPOINT_POINT.clone();
        let generator_v = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(42 as u32);

        let matrix = Vec::from([generator_u, generator_v]);
        let mut x1 = Scalar::from(12345 as u32);
        let x2 = Scalar::from(67891 as u32);
        let witness = Vec::from([x1, x2]);

        let statement = Vec::from([generator_u * x1 + generator_v * x2]);

        let mut proof = prove(&matrix, &witness, &statement).expect("generating proof should not fail");

        println!("{:?}", proof);
        println!("{:?}", verify(&matrix, &statement, &proof));
        assert!(verify(&matrix, &statement, &proof).is_ok());

        verify(&matrix, &statement, &proof).expect("proof should be valid");

        proof[1] = proof[2];

        assert!(verify(&matrix, &statement, &proof).is_err());

        x1 = x1 + Scalar::from(1 as u32);
        let bad_witness = Vec::from([x1, x2]);
        assert!(prove(&matrix, &bad_witness, &statement).is_err()) // should not prove false statements
    }

    #[test]
    fn dl_pok() {
        let matrix = Vec::from([RISTRETTO_BASEPOINT_POINT.clone()]);
        let mut x = Vec::from([Scalar::from(12345 as u32)]);
        let statement = Vec::from([RistrettoPoint::mul_base(&x[0])]);

        let mut proof = prove(&matrix, &x, &statement).expect("generating proof should not fail");

        assert!(verify(&matrix, &statement, &proof).is_ok());

        verify(&matrix, &statement, &proof).expect("proof should be valid");

        proof[0] = proof[1];

        assert!(verify(&matrix, &statement, &proof).is_err());

        x[0] = x[0] + Scalar::from(1 as u32);
        assert!(prove(&matrix, &x, &statement).is_err()) // should not prove false statements
    }
}
