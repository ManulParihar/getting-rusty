use sha2::{Digest, Sha256};

// Merkle Tree
#[derive(Clone)]
pub enum Node {
    // Leaf stores only the hash
    Leaf([u8; 32]),
    // Branch must have left and right child nodes
    Branch {
        left: Box<Node>,
        right: Box<Node>,
        hash: [u8; 32],
    }
}

impl Node {
    pub fn create(leaf_nodes: Vec<[u8; 32]>) -> Self {
        // Populate the current_row with Leaf nodes
        let mut current_row: Vec<Node> = leaf_nodes
            .into_iter()
            .map(Node::Leaf)
            .collect();
        
        // Start traversing upwards from leaf nodes to the root
        while current_row.len() > 1 {
            // In case of odd length, duplicate the last element
            if (current_row.len() % 2) != 0 {
                let last = current_row.last().unwrap().clone();
                current_row.push(last);
            }

            let mut parent_row: Vec<Node> = Vec::new();
            let mut iter = current_row.into_iter();

            // 1st iter.next() gives left node, and 2nd iter,next gives right node
            // This way, we move 2 steps at a time through the current_row
            while let Some(left_node) = iter.next() {
                let right_node = iter.next().unwrap();

                let hash1 = match &left_node {
                    Node::Leaf(h) => h,
                    Node::Branch { hash, .. } => hash,
                };

                let hash2 = match &right_node {
                    Node::Leaf(h) => h,
                    Node::Branch { hash, .. } => hash,
                };

                let parent_hash = Self::hash_nodes(hash1, hash2);
                let parent_node = Node::Branch { 
                    left: Box::new(left_node),
                    right: Box::new(right_node),
                    hash: parent_hash
                };

                parent_row.push(parent_node);
            }

            current_row = parent_row;
        }

        current_row.pop().unwrap()
    }
    
    pub fn root(&self) -> [u8; 32] {
        match self {
            Node::Leaf(h) => *h,
            Node::Branch { hash, .. } => *hash,
        }
    }

    pub fn hash_nodes(hash1: &[u8; 32], hash2: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(*hash1);
        hasher.update(*hash2);
        let result = hasher.finalize();
        result.into()
    }
}
