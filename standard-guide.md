if we break down the codebase into main pieces, we have users, validators, proofs and a state manager where the validators signature and accountfi lives

1. User accountfi - it's a user with an external wallet creates new wallet in this 2pc mpc. that neew wallet is encrypted onchain, here its saved in a state variable linked to the external wallet. The external wallet user can create a signatureed transaction, will be verified and used to verify ownership of thst encrypted wallet in the state and do the desired tx.
