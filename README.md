# Okamoto

**This library has not been security audited and should be considered insecure and experimental.**

Okamoto is a library for producing NIZK proofs of:
- knowledge of any linear relationship about discrete logarithms of group elements; and
- discrete log equivalence of group elements with respect to some generators.

That is, for a fixed $n \times m$ matrix $M$ of group elements, we consider the language of tuples $(x,Y)$ where $x$ is an $m$-dimensional vector of scalars and $Y$ is a $n$-dimensional vector of group elements such that $Mx=Y$. For DLEQ proofs with respect to generators $H_1, \ldots, H_n$, we consider the language of tuples $(x,(Y_1,\ldots,Y_n))$ where $x$ is a scalar and $Y_1,\ldots,Y_n$ are group elements such that $xH_i = Y_i$ for all $i \in [n]$.

In its current iteration the library is not generic and operates on ristretto255 group elements and scalars. The hash function used for the Fiat-Shamir transform is SHA512. For more information on the underlying general linear protocol, see Boneh-Shoup 19.5.3 (A Sigma protocol for arbitrary linear relations) and 20.3.3 (The Fiat-Shamir transform).

See the tests (`src/lib.rs`) for examples of how to prove knowledge of a DL of some group element (Schnorr), prove knowledge of a representation of a group element in terms of multiple generators (Okamoto), or do DLEQ proofs.
