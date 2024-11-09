use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

use okamoto::{prove,verify};

fn main() {
    let M = Vec::from([RISTRETTO_BASEPOINT_POINT.clone()]);
    let x = Vec::from([Scalar::from(12345 as u32)]);
    let X = Vec::from([RistrettoPoint::mul_base(&x[0])]);

    let mut proof = prove(&M, &x, &X).expect("generating proof should not fail");

    verify(&M,&X,&proof).expect("proof should be valid");

    proof[0] = proof[1];

    if let Ok(_) = verify(&M,&X,&proof) {
        panic!("proof should be invalid")
    }

    println!("{:?}", proof);
}
