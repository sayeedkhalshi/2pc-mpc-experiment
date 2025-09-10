1. Key Splitting and Distribution
   The private key of the new wallet is not stored as a single entity. Instead, it is split into shares using a secret sharing scheme (e.g., Shamir's Secret Sharing).
   Each validator receives a share of the private key.
   These shares are distributed in such a way that only a threshold number of validators (e.g., t+1 out of n) are required to reconstruct the private key or perform cryptographic operations.
2. Encryption
   Each validator encrypts its share of the private key using an additively homomorphic encryption (AHE) scheme.
   The encrypted shares are aggregated into a single ciphertext, which represents the encrypted private key. This ciphertext is saved on-chain or in a state variable linked to the external wallet that created the new wallet.
3. Validator Collaboration
   Validators do not individually hold the full private key. Instead, they hold encrypted shares of the private key.
   When a transaction needs to be signed, validators collaborate to perform the signing operation without reconstructing the private key.
   This is achieved using threshold signing protocols.
4. Security and Scalability
   The private key remains secure because it is never reconstructed in its entirety.
   The protocol ensures that even if some validators are compromised, the private key cannot be exposed unless the number of compromised validators exceeds the threshold (t).
