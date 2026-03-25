use eth_types::H256;
use sha2::{Digest, Sha256};

pub struct Merkle {
    levels: Vec<Vec<H256>>,
}

pub struct MerkleProof {
    hash: H256,
    is_left: bool,
}

impl Merkle {
    // Construct a merkle tree from a list of leaf hashes
    pub fn new(leaves: Vec<H256>) -> Self {
        let mut levels: Vec<Vec<H256>> = Vec::new();
        let mut current_row = leaves;
        
        while current_row.len() > 1 {
            let mut parent_row: Vec<H256> = Vec::new();
            // duplicate last element in case of odd number of elements
            if (current_row.len() % 2) != 0 {
                let last = current_row.last().unwrap();
                current_row.push(*last);
            }

            for i in (0..current_row.len()).step_by(2) {
                let left_hash = current_row[i];
                let right_hash = current_row[i+1];
                let parent_hash = Self::calculate_parent_hash(&left_hash, &right_hash);
                parent_row.push(parent_hash);
            }

            levels.push(current_row);
            // update current_row for next iteration
            current_row = parent_row;
        }

        // push the root element
        levels.push(current_row);

        Self { levels }
    }

    pub fn calculate_parent_hash(hash1: &H256, hash2: &H256) -> H256 {
        let mut hasher = Sha256::new();
        hasher.update(hash1.as_bytes());
        hasher.update(hash2.as_bytes());
        
        let result = hasher.finalize();
        let result_bytes: [u8; 32] = result.into();
        
        H256::from(result_bytes)
    }

    // Returns list of siblings sufficient to verify a leaf node
    pub fn get_proof(&self, index: usize) -> Vec<MerkleProof> {
        let mut proofs: Vec<MerkleProof> = Vec::new();
        let mut proof_index = index;
        
        for i in 0..(self.levels.len() - 1) {
            let nodes = &self.levels[i];

            let sibling_index = if proof_index % 2 == 0 {
                proof_index + 1
            } else {
                proof_index - 1
            };

            let sibling = if sibling_index < nodes.len() {
                nodes[sibling_index]
            } else {
                // duplicate the last element
                nodes[proof_index]
            };
            
            proofs.push(MerkleProof {
                hash: sibling,
                // can't use sibling_index because of the case where sibling_index = nodes.len()
                // In that case, sibling_index would be out of bounds (because no. of nodes will be odd)
                is_left: proof_index % 2 != 0
            });

            proof_index /= 2;
        }

        proofs
    }

    // Verifies that a leaf is part of merkle tree against a given set of proof and the root
    // NOTE: If the leaf, proof and root are all malicious, the function still returns true
    // In that case, it is upon the user to verify the input root against the correct one.
    pub fn verify_proof(leaf: H256, proof: Vec<MerkleProof>, root: H256) -> bool {
        let mut current_hash = leaf;

        for p in proof {
            current_hash = if p.is_left {
                Self::calculate_parent_hash(&p.hash, &current_hash)
            } else {
                Self::calculate_parent_hash(&current_hash, &p.hash)
            };
        }

        current_hash == root
    }
}
