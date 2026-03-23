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

    // pub fn create(leaf_nodes: Vec<[u8; 32]>) -> Self {
    //     let leaf_count = leaf_nodes.len();
    //     let mut list = leaf_nodes;
    //     // In case of odd no. of leaves, last node is duplicated
    //     if leaf_count % 2 != 0 {
    //         list.push(list[leaf_count-1]);
    //     }

    //     let mut current_row: Vec<Node> = Vec::new();
    //     let mut parent_row: Vec<Node> = Vec::new();
    //     let mut len;

    //     for i in 0..leaf_count {
    //         current_row.push(
    //             Node::Leaf(list[i])
    //         );
    //     }
    //     len = current_row.len();

    //     while len > 1 {
    //         for i in 0..(len/2) {
    //             let hash1 = match &current_row[2*i] {
    //                 Node::Leaf(hash) => hash,
    //                 Node::Branch { left: _, right: _, hash } => hash
    //             };
    //             let hash2 = match &current_row[(2*i)+1] {
    //                 Node::Leaf(hash) => hash,
    //                 Node::Branch { left: _, right: _, hash } => hash
    //             };
    //             let parent_hash = Self::hash_nodes(hash1, hash2);
                
    //             let parent_node = Node::Branch {
    //                 left: Box::new(current_row[2*i]),
    //                 right: Box::new(current_row[(2*i)+1]),
    //                 hash: parent_hash
    //             };

    //             parent_row.push(parent_node);
    //         }

    //         let mut count = parent_row.len();

    //         if (len % 2) != 0 {
    //             parent_row.push(parent_row[count-1]);
    //             count += 1;
    //         }
            
    //         len = count;
    //         current_row = parent_row;
    //         parent_row = Vec::new();
    //     }

    //     current_row[0]
    // }

    pub fn hash_nodes(hash1: &[u8; 32], hash2: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(*hash1);
        hasher.update(*hash2);
        let result = hasher.finalize();
        result.into()
    }
}
