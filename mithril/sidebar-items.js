initSidebarItems({"fn":[["concat_avk_with_msg","Serializes the Merkle Tree together with a message in a single vector of bytes. Outputs msg || avk as a vector of bytes."],["ev_lt_phi","Compares the output of `phi` (a real) to the output of `ev` (a hash). Used to determine winning lottery tickets."]],"mod":[["key_reg","Placeholder key registration functionality."],["merkle_tree","Creation and verification of Merkle Trees"],["mithril_proof","Prove the validity of aggregated signatures."],["models","Convenient instantiations MTHashLeaf"],["msp","Base multisignature scheme, used as a primitive for STM. See Section 2.4 of the paper."],["proof","General API for producing proofs from statements and witnesses"],["stm","Top-level API for Mithril Stake-based Threshold Multisignature scheme. See figure 6 of the paper for most of the protocol."]],"struct":[["Path","Path of hashes from root to leaf in a Merkle Tree. Used to verify the credentials of users and signatures."]],"type":[["Index","Quorum index for signatures. An aggregate signature (`StmMultiSig`) must have at least `k` unique indices."],["PartyId","Party identifier, unique for each participant in the protocol."],["Stake","The quantity of stake held by a party, represented as a `u64`."]]});