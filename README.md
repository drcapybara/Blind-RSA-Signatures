## Zero-Knowledge Proof Protocol for RSA Signature without Revealing the Message

**Initial Setup:**
- Alice's RSA public key is $\((n, e)\)$.
- The message $\(m\)$ is known only to the prover.

**Prover Initialize:**
- The prover knows a signature $\(s\)$ such that $\(s^e \equiv m \mod n\)$.
- The prover encrypts the signature by computing $\(v = s^e \mod n\)$ and sends $\(v\)$ to the challenger.

**The ZK Proof Process (Repeated \(k\) Times):**
1. **Prover's Randomization and Commitment:**
   - The prover picks a random number $\(r_i\)$ modulo $\(n\)$, computes $\(c_i = r_i^e\)$, and sends $\(c_i\)$ to the challenger.
2. **Challenger's Response:**
   - The challenger picks a random bit $\(b_i\)$ and sends it back to the prover.
3. **Prover's Response Depending on $\(b_i\)$:**
   - If \(b_i = 0\): The prover reveals $\(r_i\)$. The challenger verifies that \(r_i^e = c_i\).
   - If \(b_i = 1\): The prover reveals $\(z_i = s \cdot r_i \mod n\)$. The challenger verifies that $\(z_i^e = v \cdot c_i \mod n\)$.

## Verification Step in the Zero-Knowledge Proof Protocol for RSA Signature

Consider the scenario where the challenger's random bit $\(b_i = 1\)$:

**Prover's Action:**
- The prover reveals $\(z_i = s \cdot r_i \mod n\)$.

**Challenger's Verification:**
- The challenger verifies the equation $\(z_i^e \equiv v \cdot c_i \mod n\)$.
- Here, $\(v = s^e \mod n\) and \(c_i = r_i^e \mod n\)$.

**Breakdown of the Verification Step:**
1. On the left-hand side, $\((s \cdot r_i \mod n)^e\)$ simplifies to $\(s^e \cdot r_i^e \mod n\)$.
2. On the right-hand side, $\((s^e \mod n) \cdot (r_i^e \mod n)\)$ also simplifies to $\(s^e \cdot r_i^e \mod n\)$.
3. Therefore, both sides of the equation are equal, ensuring the consistency of the prover's claim.

This step in the protocol ensures that the prover can demonstrate knowledge of the signature $\(s\)$ without revealing either the signature or the message $\(m\)$, adhering to the zero-knowledge property.
