# This is how I implemented 2pc-mpc from no prior knowledge

## Strategies - 1. Learn history, 2. how developed, 3. algorithms practice, 4. security, 5. speed, 6. Writing in the mathematical foundation doc, 7. Making a YoutTube Video on it, 8. Writing a post on X, 9. Link all three here.

1. **done** Learn Division Theory
   **ongoing** Write Doc For Division Theory
   **not started** Practice algorithms For Division Theory

2. **done** Learn GCD, extended GCD
   **ongoing** Write Doc
   **not started** Practice algorithms

3. **done** Learn CRT
   **ongoing** Write Doc
   **not started** Practice algorithms

4. **done** Learn Fermat's Little Theory

5. **done** Learn Eular's Theory

6. **done** Learn Lagrange Group and congruence

7. **ongoing** What is RSA,
   **ongoing** Steps of RSA, randomess, primality test
   **ongoind** security of RSA and speed optimization
   **ongoing** Best Practices of RSA

8. Learn Polynomials,
9. Geometry of euclid, apollonious's cone
10. Kartesian Cordination
11. Algebraic Gemometry
12. Learn Symmetry
13. Group and Galois
14. Gauss
15. and all scientist one by one
16. ECC
17. DSA
18. ECDSA
19. EDDSA
20. GGN16 and 20
21. Best Practices of ECDSA, EDDSA
22. Proofs
    ZK proof, Schnorr, Maurer, Enhance Maurer, Range proof, Bulletproof, Aggregatable proof,
23. Homomorphism
24. BFT, Commitment, Pedersen commitment, padding
25. Pailliar
26. Class Group
27. Standard 2pc mpc implementation
    user - create a ECDSA wallet, encrypt private key, sign transaction
    validator - create public key, split private key, encrypt with class group, generate key shares, combine key shares, make a sihnature for consensus and keep it on state.
    fault tolerance - design for node failures
