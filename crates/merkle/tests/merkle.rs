#[cfg(test)]
mod tests {
    use eth_types::H256;
    use sha2::{Digest, Sha256};
    use merkle::{ Merkle, MerkleProof };

    fn hash_str(str: &str) -> H256 {
        let mut hasher = Sha256::new();
        hasher.update(str);
        let res: [u8; 32] = hasher.finalize().into();
        H256::from(res)
    }

    fn create_merkle(is_odd: bool) -> Merkle {
        let len = if is_odd { 9 } else { 10 };

        let mut list: Vec<H256> = Vec::new();
        for i in 0..len {
            let s = format!("merkle_proof_{}", i);
            list.push(hash_str(&s));
        }

        Merkle::new(list)
    }

    #[test]
    // valid leaf + correct proof + correct root => true
    fn valid_proof_check_odd() {
        let merkle_tree = create_merkle(true);
        
        let leaf_str = format!("merkle_proof_{}", 5);
        let leaf = hash_str(&leaf_str);

        let proofs = merkle_tree.get_proof(5);
        let root = merkle_tree.root();

        let is_proof_correct = Merkle::verify_proof(leaf, proofs, root);

        assert!(is_proof_correct);
    }

    #[test]
    // valid leaf + correct proof + correct root => true
    fn valid_proof_check_even() {
        let merkle_tree = create_merkle(false);
        
        let leaf_str = format!("merkle_proof_{}", 5);
        let leaf = hash_str(&leaf_str);

        let proofs = merkle_tree.get_proof(5);
        let root = merkle_tree.root();

        let is_proof_correct = Merkle::verify_proof(leaf, proofs, root);

        assert!(is_proof_correct);
    }

    #[test]
    // valid leaf + correct proof + incorrect root => false
    fn invalid_root() {
        let merkle_tree = create_merkle(false);
        
        let leaf_str = format!("merkle_proof_{}", 5);
        let leaf = hash_str(&leaf_str);

        let proofs = merkle_tree.get_proof(5);
        let root = H256::ZERO; // incorrect root

        let is_proof_correct = Merkle::verify_proof(leaf, proofs, root);

        assert!(!is_proof_correct);
    }

    #[test]
    // incorrect leaf + correct proof + correct root => false
    fn incorrect_leaf() {
        let merkle_tree = create_merkle(true);

        // Generate proof for leaf with index 5
        let proofs = merkle_tree.get_proof(5);
        let root = merkle_tree.root();

        let incorrect_leaf_str = format!("merkle_proof_{}", 8);
        let incorrect_leaf = hash_str(&incorrect_leaf_str);

        let is_proof_correct = Merkle::verify_proof(incorrect_leaf, proofs, root);

        assert!(!is_proof_correct);
    }

    #[test]
    // valid leaf + incorrect proof + correct root => false
    fn invalid_proof() {
        let merkle_tree = create_merkle(true);
        
        let leaf_str = format!("merkle_proof_{}", 5);
        let leaf = hash_str(&leaf_str);

        let mut proofs = merkle_tree.get_proof(5);
        // tampering with the proof
        proofs.pop();
        proofs.push(MerkleProof::new(H256::ZERO, true));

        let root = merkle_tree.root();

        let is_proof_correct = Merkle::verify_proof(leaf, proofs, root);

        assert!(!is_proof_correct);
    }

    #[test]
    // proof should be empty, verify_proof should still work
    fn single_leaf_merkle_tree() {
        let single_leaf = H256::from([1u8; 32]);
        let merkle_tree = Merkle::new(vec![H256::from([1u8; 32])]);

        let proof = merkle_tree.get_proof(0);
        assert_eq!(
            (&proof).len(),
            0
        );

        let is_proof_correct = Merkle::verify_proof(single_leaf, proof, merkle_tree.root());
        assert!(is_proof_correct);
    }
}