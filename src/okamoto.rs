use crate::errors::{ProveError, VerifyingError};
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

use sha2::{Digest, Sha512};

use rand_core::OsRng;

pub fn prove(
    matrix: &[RistrettoPoint],
    witness: &[Scalar],
    statement: &[RistrettoPoint],
) -> Result<Vec<Scalar>, ProveError> {
    if matrix.len() != witness.len() * statement.len() {
        return Err(ProveError::InvalidDimensions);
    }

    let M = witness.len();
    let N = statement.len();

    let recomputed_statement: Vec<RistrettoPoint> = (0..N).map(|i| (0..M).map(|j| matrix[i*N+j] * witness[j]).sum::<RistrettoPoint>()).collect();

    if recomputed_statement != statement {
        return Err(ProveError::Unsound);
    }
    
    let mut csprng = OsRng;
    let commitment_trapdoors: Vec<Scalar> = (0..M).map(|_| Scalar::random(&mut csprng)).collect();

    let commitments: Vec<RistrettoPoint> = (0..N)
        .map(|i| {
            (0..M)
                .map(|j| commitment_trapdoors[j] * matrix[i * N + j])
                .sum()
        })
        .collect();

    let challenge = compute_challenge(matrix, statement, &commitments);

    let mut proof = Vec::from([challenge]);
    proof.extend((0..M).map(|i| (commitment_trapdoors[i] + challenge * witness[i])));

    Ok(proof)
}

fn compute_challenge(
    matrix: &[RistrettoPoint],
    statement: &[RistrettoPoint],
    commitments: &[RistrettoPoint],
) -> Scalar {
    let mut hash = Sha512::new();

    for compressed_point in RistrettoPoint::double_and_compress_batch(&Vec::from(
        [matrix, statement, commitments].concat(),
    )) {
        hash.update(compressed_point.as_bytes());
    }

    Scalar::from_hash(hash)
}

pub fn verify(
    matrix: &[RistrettoPoint],
    statement: &[RistrettoPoint],
    proof: &[Scalar],
) -> Result<(), VerifyingError> {
    let N = statement.len();
    let M = proof.len() - 1;

    let challenge = proof[0];
    let responses = &proof[1..];

    // first step: recompute R_i's for all i in [m]
    let recomputed_commitments: Vec<RistrettoPoint> = (0..N)
        .map(|i| {
            (0..N)
                .map(|j| responses[j] * matrix[i * N + j])
                .sum::<RistrettoPoint>()
                - challenge * statement[i]
        })
        .collect();

    let recomputed_challenge = compute_challenge(&matrix,&statement,&recomputed_commitments);

    if recomputed_challenge == proof[0] {
        return Ok(()) 
    }

    Err(VerifyingError::Invalid)
}
