#[cfg(feature = "hash")]
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

            #[cfg(feature = "hash")]
            let hash = crypto::as_sha256(value.as_bytes()).to_string();
            #[cfg(not(feature = "hash"))]
            let hash = value;
            let node = MerkleNode::new(
                hash,
                Some(Box::new(curr.clone())),
                next.map(Clone::clone).map(Box::new),
            );

            result.push(node);
        }

        Self::root(result)
    }

    #[cfg(not(feature = "hash"))]
    pub fn from_hash_list(hash_list: Vec<String>) -> Self {
        let node_list = hash_list.into_iter().map(Self::empty_new).collect();

        Self::root(node_list)
    }

    #[cfg(feature = "hash")]
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

impl AsRef<MerkleNode> for LocationNode {
    fn as_ref(&self) -> &MerkleNode {
        use LocationNode::*;

        match self {
            Root(node) | Left(node) | Right(node) => node,
        }
    }
}

impl LocationNode {
    pub fn hash(&self) -> &str {
        &self.as_ref().hash
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
                let tree = MerkleTree::new(*node.clone());
                tree.find_sibling_of(hash)
            }
            (Some(left), Some(right)) => {
                if left.hash == hash {
                    return Some(Right(*right.clone()));
                }

                if right.hash == hash {
                    return Some(Left(*left.clone()));
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

    pub fn verify(&self, hash: String) -> bool {
        let mut sibling = self.find_sibling_of(hash.clone());

        let mut hash = hash;
        while let Some(node) = &sibling {
            let combine_text = match node {
                LocationNode::Root(node) => return node.hash == hash,
                LocationNode::Left(node) => format!("{}{}", node.hash, hash),
                LocationNode::Right(node) => format!("{}{}", hash, node.hash),
            };

            #[cfg(feature = "hash")]
            {
                hash = crypto::as_sha256(combine_text.as_bytes()).to_string();
            };

            #[cfg(not(feature = "hash"))]
            {
                hash = combine_text;
            };

            sibling = self.find_sibling_of(hash.clone());
        }

        sibling.map(|node| node.hash() == hash).unwrap_or(false)
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

        #[cfg(feature = "hash")]
        assert_hash_eq!(
            EVEN_CASE,
            "24A5154C51E5EA8F72DBFCD82F42202F94BB34E1A1D764814D0A0BE012DD866E"
        );
        #[cfg(not(feature = "hash"))]
        assert_hash_eq!(EVEN_CASE, "ABCDEFGH");

        #[cfg(feature = "hash")]
        assert_hash_eq!(
            ODD_CASE,
            "AE4F3A195A3CBD6A3057C205DEF94520930F03F51F73C5A540D8FDAB05163FEF"
        );
        #[cfg(not(feature = "hash"))]
        assert_hash_eq!(ODD_CASE, "ABCDE");
    }

    #[test]
    fn find_sibling_of() {
        macro_rules! find_sibling {
            ($tree: expr, $left: expr, $right: expr) => {{
                #[cfg(feature = "hash")]
                let hash = crypto::as_sha256($left.as_bytes()).to_string();
                #[cfg(not(feature = "hash"))]
                let hash = $left.to_owned();

                let sibling = $tree
                    .find_sibling_of(hash.clone())
                    .expect(&format!("not found {hash} in merkle tree"));

                #[cfg(feature = "hash")]
                assert_eq!(
                    sibling.hash(),
                    crypto::as_sha256($right.as_bytes()).to_string()
                );
                #[cfg(not(feature = "hash"))]
                assert_eq!(sibling.hash(), $right);

                sibling
            }};
        }

        macro_rules! not_found_sibling {
            ($tree: expr, $origin: expr) => {
                #[cfg(feature = "hash")]
                let hash = crypto::as_sha256($origin.as_bytes()).to_string();
                #[cfg(not(feature = "hash"))]
                let hash = $origin.to_owned();
                let sibling = $tree.find_sibling_of(hash.clone());

                assert!(sibling.is_none(), "found {hash} in merkle tree")
            };
        }

        macro_rules! assert_node_eq {
            ($tree: expr, $left: expr, $right: expr) => {
                let sibling = find_sibling!($tree, $left, $right);
                let node = sibling.as_ref();

                assert_eq!(node.left, None);
                assert_eq!(node.right, None);
            };
        }

        let node = generate_root_node!(EVEN_CASE);
        let tree: MerkleTree = node.into();

        assert_node_eq!(tree, "A", "B");
        assert_node_eq!(tree, "B", "A");
        assert_node_eq!(tree, "C", "D");
        assert_node_eq!(tree, "D", "C");

        not_found_sibling!(tree, "Hello");

        let node = generate_root_node!(ODD_CASE);
        let tree: MerkleTree = node.into();

        assert_node_eq!(tree, "A", "B");
        assert_node_eq!(tree, "B", "A");
        assert_node_eq!(tree, "C", "D");
        assert_node_eq!(tree, "D", "C");

        not_found_sibling!(tree, "Hello");
    }

    #[test]
    fn verify() {
        macro_rules! verify {
            ($case: expr) => {
                let node = generate_root_node!($case);
                let tree: MerkleTree = node.into();

                for origin in $case {
                    #[cfg(feature = "hash")]
                    let hash = crypto::as_sha256(origin.as_bytes()).to_string();
                    #[cfg(not(feature = "hash"))]
                    let hash = origin.to_owned();

                    assert!(tree.verify(hash.clone()), "failed to verify hash(hash)");
                }
            };
        }

        verify!(ODD_CASE);
        verify!(EVEN_CASE);
    }
}
