mod errors;
mod okamoto;

pub use crate::errors::*;
pub use crate::okamoto::*;

use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dl_repr() {
            let generator_U = RISTRETTO_BASEPOINT_POINT.clone();
            let generator_V = RISTRETTO_BASEPOINT_POINT.clone() * 42;

            let M = Vec::from([generator_U, generator_V]);
            let mut x1 = Vec::from([Scalar::from(12345 as u32)]);
            let mut x2 = Vec::from([Scalar::from(67891 as u32)]);
            let witness = Vec::from([x1, x2]);

            let X = Vec::from([generatorU * x1 + generatorV * x2);

            let mut proof = prove(&M, &witness, &X).expect("generating proof should not fail");

            assert!(verify(&M,&X,&proof).is_ok());

            verify(&M,&X,&proof).expect("proof should be valid");

            proof[1] = proof[2];

            assert!(verify(&M,&X,&proof).is_err());

            x1[0] = x1[0] + Scalar::from(1 as u32);
            witness = Vec::from([x1, x2]);
            assert!(prove(&M, &witness, &X).is_err()) // should not prove false statements
    }


    #[test]
    fn dl_pok() {
            let M = Vec::from([RISTRETTO_BASEPOINT_POINT.clone()]);
            let mut x = Vec::from([Scalar::from(12345 as u32)]);
            let X = Vec::from([RistrettoPoint::mul_base(&x[0])]);

            let mut proof = prove(&M, &x, &X).expect("generating proof should not fail");

            assert!(verify(&M,&X,&proof).is_ok());

            verify(&M,&X,&proof).expect("proof should be valid");

            proof[0] = proof[1];

            assert!(verify(&M,&X,&proof).is_err());

            x[0] = x[0] + Scalar::from(1 as u32);
            assert!(prove(&M, &x, &X).is_err()) // should not prove false statements
    }



    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
