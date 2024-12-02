use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

use rand_core::OsRng;

use crate::okamoto::compute_challenge;
use crate::errors::{ProveError,VerifyingError};

#[cfg(feature="check_soundness")]
fn dleq_sound(generators: &[RistrettoPoint], witness: &Scalar, statement: &[RistrettoPoint]) -> bool {
    for i in 0..generators.len() {
        if witness * generators[i] != statement[i] {
            return false     
        }
    }
    
    true
}

#[cfg(not(feature="check_soundness"))]
fn dleq_sound(_generators: &[RistrettoPoint], _witness: &Scalar, _statement: &[RistrettoPoint]) -> bool {
    true
}

pub fn prove_dleq(generators: &[RistrettoPoint], witness: &Scalar, statement: &[RistrettoPoint]) -> Result<Vec<Scalar>,ProveError> {
    let n_statement_dim = statement.len();

    if n_statement_dim != generators.len() {
        return Err(ProveError::InvalidDimensions);
    }

    if !dleq_sound(generators, witness, statement) {
        return Err(ProveError::Unsound);
    }

    let mut csprng = OsRng;

    let commitment_trapdoor = Scalar::random(&mut csprng);

    let commitments: Vec<RistrettoPoint> = generators.into_iter().map(|g| commitment_trapdoor * g).collect();

    let challenge = compute_challenge(&generators, &statement, &commitments);

    let proof = Vec::from([challenge, challenge * witness + commitment_trapdoor]);

    Ok(proof)
}

pub fn verify_dleq(generators: &[RistrettoPoint], statement: &[RistrettoPoint], proof: &[Scalar]) -> Result<(), VerifyingError> {
    let n_statement_dim = statement.len();

    if n_statement_dim != generators.len() {
        return Err(VerifyingError::Malformed);
    }

    let challenge = proof[0];
    let response = proof[1];

    let recomputed_commitments: Vec<RistrettoPoint> = (0..n_statement_dim).map(|i| response * generators[i] - challenge * statement[i]).collect();

    let recomputed_challenge = compute_challenge(&generators, &statement, &recomputed_commitments);

    if recomputed_challenge == challenge {
        return Ok(());
    }

    Err(VerifyingError::Invalid)
}
