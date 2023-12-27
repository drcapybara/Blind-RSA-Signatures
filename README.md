## Blind RSA Signatures/Messages

**Initial Setup:**
- Alice's RSA public key is $\(n, e\)$.
- The message $\(m^e\)$ is publicly known.

**The ZK Proof Process (Repeated \(k\) Times):**
1. **Prover's Randomization and Commitment:**
   - The prover picks a random number $\(r_i\)$ modulo $\(n\)$, computes $\(c_i = r_i^e\)$, and sends $\(c_i\)$ to the challenger.
2. **Challenger's Response:**
   - The challenger picks a random bit $\(b_i\)$ and sends it back to the prover.
3. **Prover's Response Depending on $\(b_i\)$:**
   - If $\(b_i = 0\)$: The prover reveals $\(r_i\)$. The challenger verifies that $\(r_i^e = c_i\)$.
   - If $\(b_i = 1\)$: The prover reveals $\(z_i = m \cdot r_i \mod n\)$. The challenger verifies that $\(z_i^e = m^e \cdot c_i \mod n\)$.
