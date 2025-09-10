---
Folder structure
---

src/
├─ main.rs # Entry point
├─ user/
│ ├─ mod.rs # User module
│ ├─ wallet.rs # Wallet struct, creation, serialization
│ └─ mpc_interaction.rs # Functions to request MPC wallet creation
├─ validators/
│ ├─ mod.rs # Validators module
│ ├─ secret_sharing.rs # Shamir split & reconstruction
│ ├─ consensus.rs # Signature aggregation & verification
│ └─ class_group_engine.rs # Class group implementation
├─ proofs/
│ └─ zk_proofs.rs # Schnorr, Pedersen, bulletproofs (placeholder)
└─ state/
└─ state_manager.rs # Persistent state of wallets and signatures
