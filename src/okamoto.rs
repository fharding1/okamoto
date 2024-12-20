use crate::errors::{ProveError, VerifyingError};
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

use sha2::{Digest, Sha512};

use rand_core::OsRng;

#[cfg(feature = "check_soundness")]
fn linear_sound(
    matrix: &[RistrettoPoint],
    witness: &[Scalar],
    statement: &[RistrettoPoint],
) -> bool {
    let n_statement_dim = statement.len();
    let m_witness_dim = witness.len();

    // recompute the statement from the matrix and witness so that we can check soundness
    let recomputed_statement: Vec<RistrettoPoint> = (0..n_statement_dim)
        .map(|i| {
            (0..m_witness_dim)
                .map(|j| matrix[i * n_statement_dim + j] * witness[j])
                .sum::<RistrettoPoint>()
        })
        .collect();

    recomputed_statement == statement
}

#[cfg(not(feature = "check_soundness"))]
fn linear_sound(
    _matrix: &[RistrettoPoint],
    _witness: &[Scalar],
    _statement: &[RistrettoPoint],
) -> bool {
    true
}

pub fn prove_linear(
    matrix: &[RistrettoPoint],
    witness: &[Scalar],
    statement: &[RistrettoPoint],
) -> Result<Vec<Scalar>, ProveError> {
    let n_statement_dim = statement.len();
    let m_witness_dim = witness.len();

    // check that matrix is m times n
    if matrix.len() != m_witness_dim * n_statement_dim {
        return Err(ProveError::InvalidDimensions);
    }

    // don't prove false statements
    if !linear_sound(matrix, witness, statement) {
        return Err(ProveError::Unsound);
    }

    let mut csprng = OsRng;
    let commitment_trapdoors: Vec<Scalar> = (0..m_witness_dim)
        .map(|_| Scalar::random(&mut csprng))
        .collect();

    // compute R_i = \sum_{j=1}^m (r_j M_{i,j}) for all i in [n]
    let commitments: Vec<RistrettoPoint> = (0..n_statement_dim)
        .map(|i| {
            (0..m_witness_dim)
                .map(|j| commitment_trapdoors[j] * matrix[i * n_statement_dim + j])
                .sum()
        })
        .collect();

    // fiat shamir: compute hash of matrix, statement, commitments
    let challenge = compute_challenge(matrix, statement, &commitments);

    // proof is (c, s_1, ..., s_m)
    let mut proof = Vec::from([challenge]);
    proof.extend((0..m_witness_dim).map(|i| (commitment_trapdoors[i] + challenge * witness[i])));

    Ok(proof)
}

pub(crate) fn compute_challenge(
    matrix: &[RistrettoPoint],
    statement: &[RistrettoPoint],
    commitments: &[RistrettoPoint],
) -> Scalar {
    let mut hash = Sha512::new();

    // double-and-compress because it is batchable and does not effect security
    for compressed_point in RistrettoPoint::double_and_compress_batch(&Vec::from(
        [matrix, statement, commitments].concat(),
    )) {
        hash.update(compressed_point.as_bytes());
    }

    Scalar::from_hash(hash)
}

pub fn verify_linear(
    matrix: &[RistrettoPoint],
    statement: &[RistrettoPoint],
    proof: &[Scalar],
) -> Result<(), VerifyingError> {
    let n_statement_dim = statement.len();
    let m_witness_dim = proof.len() - 1;

    if matrix.len() != n_statement_dim * m_witness_dim {
        return Err(VerifyingError::Malformed);
    }

    let challenge = proof[0];
    let responses = &proof[1..];

    // first step: recompute R_i's for all i in [m]
    let recomputed_commitments: Vec<RistrettoPoint> = (0..n_statement_dim)
        .map(|i| {
            (0..m_witness_dim)
                .map(|j| responses[j] * matrix[i * n_statement_dim + j])
                .sum::<RistrettoPoint>()
                - challenge * statement[i]
        })
        .collect();

    let recomputed_challenge = compute_challenge(&matrix, &statement, &recomputed_commitments);

    match recomputed_challenge == challenge {
        true => Ok(()),
        false => Err(VerifyingError::Invalid),
    }
}
