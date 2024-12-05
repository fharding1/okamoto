mod dleq;
mod errors;
mod okamoto;

pub use crate::dleq::*;
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
        let x1 = Scalar::from(12345 as u32);
        let x2 = Scalar::from(67891 as u32);
        let witness = Vec::from([x1, x2]);

        let statement = Vec::from([generator_u * x1 + generator_v * x2]);

        let mut proof =
            prove_linear(&matrix, &witness, &statement).expect("generating proof should not fail");

        assert!(verify_linear(&matrix, &statement, &proof).is_ok());

        verify_linear(&matrix, &statement, &proof).expect("proof should be valid");

        proof[1] = proof[2];

        assert!(verify_linear(&matrix, &statement, &proof).is_err());
    }

    #[test]
    #[cfg(feature = "check_soundness")]
    fn dl_repr_check_soundness() {
        let generator_u = RISTRETTO_BASEPOINT_POINT.clone();
        let generator_v = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(42 as u32);

        let matrix = Vec::from([generator_u, generator_v]);
        let x1 = Scalar::from(12345 as u32);
        let x2 = Scalar::from(67891 as u32);

        let statement = Vec::from([generator_u * x1 + generator_v * x2]);

        let bad_witness = Vec::from([Scalar::from(1 as u32), x2]);
        assert!(prove_linear(&matrix, &bad_witness, &statement).is_err()) // should not prove false statements
    }

    #[test]
    fn dl_pok() {
        let matrix = Vec::from([RISTRETTO_BASEPOINT_POINT.clone()]);
        let x = Vec::from([Scalar::from(12345 as u32)]);
        let statement = Vec::from([RistrettoPoint::mul_base(&x[0])]);

        let mut proof =
            prove_linear(&matrix, &x, &statement).expect("generating proof should not fail");

        assert!(verify_linear(&matrix, &statement, &proof).is_ok());

        verify_linear(&matrix, &statement, &proof).expect("proof should be valid");

        proof[0] = proof[1];

        assert!(verify_linear(&matrix, &statement, &proof).is_err());
    }

    #[test]
    #[cfg(feature = "check_soundness")]
    fn dl_pok_check_soundness() {
        let matrix = Vec::from([RISTRETTO_BASEPOINT_POINT.clone()]);
        let x = Vec::from([Scalar::from(12345 as u32)]);
        let statement = Vec::from([RistrettoPoint::mul_base(&(x[0] + Scalar::from(1 as u32)))]);
        assert!(prove_linear(&matrix, &x, &statement).is_err()) // should not prove false statements
    }

    #[test]
    fn basic_dleq() {
        let generator = RISTRETTO_BASEPOINT_POINT.clone();
        let generator_h = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(42 as u32);

        let generators = [generator, generator_h];
        let witness = Scalar::from(10 as u32);
        let statement = Vec::from([generator * witness, generator_h * witness]);

        let mut proof = prove_dleq(&generators, &witness, &statement)
            .expect("generating proof should not fail");

        assert!(verify_dleq(&generators, &statement, &proof).is_ok());

        proof[0] = proof[1];

        assert!(verify_dleq(&generators, &statement, &proof).is_err());
    }

    #[test]
    #[cfg(feature = "check_soundness")]
    fn basic_dleq_check_soundness() {
        let generator = RISTRETTO_BASEPOINT_POINT.clone();
        let generator_h = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(42 as u32);

        let generators = [generator, generator_h];
        let witness = Scalar::from(10 as u32);
        let mut statement = Vec::from([generator * witness, generator_h * witness]);

        statement[1] = generator;
        assert!(prove_dleq(&generators, &witness, &statement).is_err()); // should not prove false
                                                                         // statements
    }

    #[test]
    fn multi_dleq() {
        let generator = RISTRETTO_BASEPOINT_POINT.clone();
        let generator_h1 = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(42 as u32);
        let generator_h2 = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(32 as u32);
        let generator_h3 = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(3 as u32);

        let generators = [generator, generator_h1, generator_h2, generator_h3];
        let witness = Scalar::from(10 as u32);
        let statement: Vec<RistrettoPoint> = generators.into_iter().map(|g| g * witness).collect();

        let mut proof = prove_dleq(&generators, &witness, &statement)
            .expect("generating proof should not fail");

        assert!(verify_dleq(&generators, &statement, &proof).is_ok());

        proof[1] = proof[1] + Scalar::from(1 as u32);

        assert!(verify_dleq(&generators, &statement, &proof).is_err());
    }

    #[test]
    #[cfg(feature = "check_soundness")]
    fn multi_dleq_check_soundness() {
        let generator = RISTRETTO_BASEPOINT_POINT.clone();
        let generator_h1 = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(42 as u32);
        let generator_h2 = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(32 as u32);
        let generator_h3 = RISTRETTO_BASEPOINT_POINT.clone() * Scalar::from(3 as u32);

        let generators = [generator, generator_h1, generator_h2, generator_h3];
        let witness = Scalar::from(10 as u32);
        let statement: Vec<RistrettoPoint> = generators
            .into_iter()
            .map(|g| g * witness * Scalar::from(2 as u32))
            .collect();

        assert!(prove_dleq(&generators, &witness, &statement).is_err()); // should not prove false
                                                                         // statements
    }
}
