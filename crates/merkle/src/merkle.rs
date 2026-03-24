use eth_types::H256;
use sha2::{Digest, Sha256};

pub struct Merkle {
    levels: Vec<Vec<H256>>
}

impl Merkle {
    pub fn new(leaves: Vec<H256>) -> Self {
        let mut levels: Vec<Vec<H256>> = Vec::new();
        let mut current_row = leaves;
        
        while current_row.len() > 1 {
            let mut parent_row: Vec<H256> = Vec::new();
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
            current_row = parent_row;
        }

        Self { levels }
    }

    pub fn calculate_parent_hash(hash1: &H256, hash2: &H256) -> H256 {
        let mut hasher = Sha256::new();
        hasher.update(*hash1);
        hasher.update(*hash2);
        
        let result = hasher.finalize();
        let result_bytes: [u8; 32] = result.into();
        
        H256::from(result_bytes)
    }
}