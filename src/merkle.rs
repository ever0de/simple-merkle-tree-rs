use crate::crypto;

#[derive(Clone, Debug, PartialEq)]
pub struct MerkleNode {
    pub hash: String,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    pub fn new(
        hash: String,
        left: Option<Box<MerkleNode>>,
        right: Option<Box<MerkleNode>>,
    ) -> Self {
        Self { hash, left, right }
    }

    pub fn empty_new(hash: String) -> Self {
        Self::new(hash, None, None)
    }

    pub fn root(node_list: Vec<MerkleNode>) -> Self {
        let length = node_list.len();

        if length == 1 {
            return node_list[0].clone();
        }

        let mut result = vec![];
        for i in (0..length).step_by(2) {
            let curr = &node_list[i];

            if i + 1 >= length {
                result.push(curr.clone());
                break;
            }

            let next = node_list.get(i + 1);

            let value = format!(
                "{}{}",
                &curr.hash,
                &next.map(|node| &node.hash).unwrap_or(&String::new())
            );
            let hash = crypto::as_sha256(value.as_bytes()).to_string();
            let node = MerkleNode::new(
                hash,
                Some(Box::new(curr.clone())),
                next.map(Clone::clone).map(Box::new),
            );

            result.push(node);
        }

        Self::root(result)
    }

    pub fn from_hash_list(hash_list: Vec<String>) -> Self {
        let node_list = hash_list
            .iter()
            .map(|hash| hash.as_bytes())
            .map(crypto::as_sha256)
            .map(|hash| hash.to_string())
            .map(Self::empty_new)
            .collect();

        Self::root(node_list)
    }
}

#[derive(Clone, Debug)]
pub struct MerkleTree {
    pub root: MerkleNode,
    // TODO: implement this
    // pub size: usize,
}

#[derive(Clone, Debug)]
pub enum LocationNode {
    Root(MerkleNode),
    Left(MerkleNode),
    Right(MerkleNode),
}

impl LocationNode {
    pub fn node(&self) -> &MerkleNode {
        use LocationNode::*;

        match self {
            Root(node) | Left(node) | Right(node) => node,
        }
    }

    pub fn hash(&self) -> &str {
        &self.node().hash
    }

    pub fn cmp_hash(&self, other: String) -> bool {
        self.hash() == other
    }
}

impl From<MerkleNode> for MerkleTree {
    fn from(root: MerkleNode) -> Self {
        Self::new(root)
    }
}

impl MerkleTree {
    pub fn new(root: MerkleNode) -> Self {
        Self { root }
    }

    pub fn find_sibling_of(&self, hash: String) -> Option<LocationNode> {
        use LocationNode::*;

        if self.root.hash == hash {
            return Some(Root(self.root.clone()));
        }

        match (&self.root.left, &self.root.right) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => {
                if node.hash == hash {
                    if self.root.left.is_some() {
                        Some(Left(*node.clone()))
                    } else {
                        Some(Right(*node.clone()))
                    }
                } else {
                    let tree = MerkleTree::new(*node.clone());
                    tree.find_sibling_of(hash)
                }
            }
            (Some(left), Some(right)) => {
                if left.hash == hash {
                    return Some(Left(*left.clone()));
                }

                if right.hash == hash {
                    return Some(Right(*right.clone()));
                }

                let tree = MerkleTree::new(*left.clone());
                match tree.find_sibling_of(hash.clone()) {
                    Some(node) => Some(node),
                    None => {
                        let tree = MerkleTree::new(*right.clone());
                        tree.find_sibling_of(hash)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EVEN_CASE: [&'static str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];
    const ODD_CASE: [&'static str; 5] = ["A", "B", "C", "D", "E"];

    macro_rules! generate_root_node {
        ($hash_list: expr) => {
            MerkleNode::from_hash_list($hash_list.into_iter().map(ToOwned::to_owned).collect())
        };
    }

    #[test]
    fn generate_root_node() {
        macro_rules! assert_hash_eq {
            ($actual: expr, $expected: expr) => {
                let node = generate_root_node!($actual);

                assert_eq!(node.hash, $expected);
            };
        }

        assert_hash_eq!(
            EVEN_CASE,
            "24A5154C51E5EA8F72DBFCD82F42202F94BB34E1A1D764814D0A0BE012DD866E"
        );

        assert_hash_eq!(
            ODD_CASE,
            "AE4F3A195A3CBD6A3057C205DEF94520930F03F51F73C5A540D8FDAB05163FEF"
        );
    }

    #[test]
    fn find_sibling_of() {
        macro_rules! find_sibling {
            ($tree: expr, $origin: expr) => {{
                let hash = crypto::as_sha256($origin.as_bytes()).to_string();
                let sibling = $tree
                    .find_sibling_of(hash.clone())
                    .expect(&format!("not found {hash} in merkle tree"));
                assert_eq!(sibling.hash(), hash);

                sibling
            }};
        }

        macro_rules! not_found_sibling {
            ($tree: expr, $origin: expr) => {
                let hash = crypto::as_sha256($origin.as_bytes()).to_string();
                let sibling = $tree.find_sibling_of(hash.clone());

                assert!(sibling.is_none(), "found {hash} in merkle tree")
            };
        }

        macro_rules! assert_node_eq {
            ($tree: expr, $origin: expr) => {
                let sibling = find_sibling!($tree, $origin);
                let node = sibling.node();

                assert_eq!(node.left, None);
                assert_eq!(node.right, None);
            };
        }

        let node = generate_root_node!(EVEN_CASE);
        let tree: MerkleTree = node.into();

        for origin in EVEN_CASE {
            assert_node_eq!(tree, origin);
        }

        not_found_sibling!(tree, "Hello");

        let node = generate_root_node!(ODD_CASE);
        let tree: MerkleTree = node.into();

        for origin in ODD_CASE {
            assert_node_eq!(tree, origin);
        }

        not_found_sibling!(tree, "Hello");
    }
}
