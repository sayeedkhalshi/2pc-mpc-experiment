# 2pc Mpc Decentralized Authentication Server

This is the right way to tackle a **complex threshold signature / MPC system**: design the architecture fully first, define all the moving parts, and then implement module by module. Let’s do a **structured, step-by-step architectural design** for a **2PC/threshold ECDSA-like MPC** system using **class-group homomorphic encryption**, ZK proofs, and fault-tolerant state handling. I’ll keep in mind **quantum-safe homomorphism** and a **BFT-like verification flow**.

---

## **1. High-level architecture**

We have **two layers**:

### **A. User layer**

-   User creates wallet (private key material is **split using threshold secret sharing**).
-   User submits a transaction request to the MPC engine.
-   User never sees the full secret; only interacts with MPC outputs.

### **B. MPC / Network layer**

-   Simulates 2 parties (can generalize to n parties if needed).
-   Each party holds a share of the secret.
-   Parties perform **joint computation**:

    -   Nonce generation (random k per signature)
    -   Partial signature computation
    -   Homomorphic aggregation using **class-group encryption**
    -   ZK proofs to prove correct partial computations:

        -   **Schnorr** (proof of knowledge of secret exponent)
        -   **Maurer / Enhanced Maurer** (bounds, correctness of operations)
        -   **Pedersen commitments** for coefficients
        -   **Range proofs / Bulletproofs** for secret ranges
        -   **Aggregatable proofs** for combined signatures

-   Parties generate a **joint signature** without revealing secret shares.
-   Final signature stored in **state** for verification and reuse.

---

## **2. Data structures**

```text
Wallet {
    user_id
    public_address
    partial_keys: Vec<SecretShare>   # one per party
}

SecretShare {
    party_id
    share_value: BigInt / ClassGroupForm
    commitments: Vec<ClassGroupForm>  # Pedersen commitments
    proofs: Vec<ZKProof>             # Schnorr, Bulletproof, etc
}

Transaction {
    tx_id
    sender: Wallet
    payload: TransactionData
    signature: MPCSignature
}

MPCSignature {
    r: BigInt / ClassGroupForm
    s: BigInt / ClassGroupForm
    proofs: Vec<AggregatableZKProof>
}
```

---

## **3. Core modules**

| Module                   | Responsibilities                                                      | Notes                                                        |
| ------------------------ | --------------------------------------------------------------------- | ------------------------------------------------------------ |
| **SecretSharing**        | Split private key into n shares using Shamir / Lagrange interpolation | Threshold t ≤ n                                              |
| **ClassGroupEngine**     | Homomorphic encryption and decryption                                 | Quantum-safe additive homomorphism, supports rerandomization |
| **PartialSigner**        | Each party computes partial signature share                           | Uses homomorphic ops; generates ZK proofs for correctness    |
| **NonceGen**             | Generates per-signature ephemeral scalar `k`                          | Supports ZK range proofs; avoids reuse                       |
| **Commitments**          | Pedersen, Maurer, Enhanced Maurer                                     | Commitments for each partial computation and secret share    |
| **Proofs**               | Schnorr, Bulletproof, Aggregatable proofs                             | Verifies correctness without revealing secrets               |
| **Aggregator**           | Combine partial signatures into final signature                       | Lagrange coefficients, homomorphic aggregation               |
| **StateManager**         | Stores partial keys, partial signatures, proofs                       | Reusable state; fault-tolerant, BFT-consensus style          |
| **TransactionValidator** | Verifies signature using stored MPC state                             | Can reject invalid shares or proofs                          |

---

## **4. Flow: 2PC MPC signature generation**

1. **Initialization**

    - Generate class-group parameters $D, g$.
    - Split wallet private key $d$ into shares $d_1, d_2$ using Shamir.
    - Each party stores its share along with commitments + ZK proofs.

2. **Nonce generation**

    - Each party generates ephemeral $k_i$ (hidden).
    - Commitments $K_i = g^{k_i}$ are published.
    - ZK range proofs verify that $k_i$ are within safe bounds.

3. **Partial signature computation**

    - Each party computes partial `s_i` using:

        $$
        s_i = k_i^{-1}(z + r \cdot d_i) \mod q
        $$

    - Encrypt `s_i` using **class-group homomorphic scheme**.
    - Generate ZK proofs for correctness (Schnorr, Pedersen).

4. **Aggregation**

    - Homomorphically combine `s_i`:

        $$
        s = \sum_i s_i \mod q
        $$

    - Decrypt final `s` using collective decryption (or combine `r, s` in class group).

5. **Verification**

    - Each party verifies:

        - Correctness of `s_i` using commitments.
        - ZK proofs pass.

    - If valid, signature `(r, s)` is stored in **state**.

6. **Reuse / consensus**

    - Signature can be reused for transaction verification.
    - Nodes or server checks stored proofs instead of recomputing full MPC.

---

## **5. Fault tolerance / BFT design**

-   Threshold $t = 1$ for 2PC (single failure tolerated) — generalize to n parties.
-   Invalid share detection:

    -   Check ZK proofs per party.
    -   If invalid, accuse + optionally reveal share proof.

-   Aggregation only proceeds if ≥ t+1 valid shares.
-   StateManager logs every step; enables replay attack prevention and auditing.
-   Optional: Combine with classical BFT consensus for network-wide validation.

---

## **6. ZK proof plan**

| Proof               | Purpose                                                 |
| ------------------- | ------------------------------------------------------- |
| Schnorr             | Prove knowledge of secret exponent (partial key, nonce) |
| Maurer              | Prove `s_i` in valid range                              |
| Enhanced Maurer     | Provide robustness against malicious shares             |
| Pedersen commitment | Commit to polynomial coefficients or ephemeral values   |
| Bulletproof         | Range proof of secret values / nonce without leaking    |
| Aggregatable        | Combine proofs efficiently across parties               |

---

## **7. Code module breakdown**

We will implement in **stages**:

1. **ClassGroupEngine**

    - Homomorphic encryption/decryption
    - Rerandomization
    - Composition + scalar exponentiation

2. **SecretSharing**

    - Shamir split + reconstruction
    - Commitments to shares (Pedersen)

3. **PartialSigner**

    - Compute `s_i`
    - Produce ZK proofs

4. **Aggregator**

    - Combine partial signatures (homomorphic sum)
    - Verify proofs

5. **TransactionHandler / StateManager**

    - Store wallet shares, MPC signatures
    - Validate transactions

6. **Tests / Simulation**

    - Simulate 2 parties
    - Ensure full end-to-end MPC signature
    - Verify correctness + fault detection

---

### ✅ **Key Design Points**

-   All secrets are **never reconstructed in clear**.
-   Use **class-group homomorphism** for quantum-safe additive aggregation.
-   ZK proofs enforce correctness for each party.
-   State can be **saved and reused** for multiple transactions.
-   Easily extendable to **n-party threshold**.
-   Minimal external dependencies; only cryptography/BigInt libraries used.
