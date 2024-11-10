# Okamoto

**This library has not been security audited and should be considered insecure and experimental.**

Okamoto is a library for producing NIZK proofs of knowledge of any linear relationship about discrete logarithms of ristretto255 group elements. That is, for a fixed n by m matrix M of group elements, we consider the language of tuples (x,H) where x is an m-dimensional vector of scalars and H is a n-dimensional vector of group elements such that Mx=H. The hash function used for the Fiat-Shamir transform is SHA512. For more information on the underlying protocol, see Boneh-Shoup 19.5.3 (A Sigma protocol for arbitrary linear relations) and 20.3.3 (The Fiat-Shamir transform).

See the tests (`src/lib.rs`) for examples of how to prove knowledge of a DL of some group element (Schnorr) or prove knowledge of a representation of a group element in terms of multiple generators (Okamoto).
