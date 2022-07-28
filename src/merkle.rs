use crate::crypto;

#[derive(Clone, Debug)]
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

impl MerkleTree {
    pub fn new(root: MerkleNode) -> Self {
        Self { root }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_root_node() {
        macro_rules! assert_hash_eq {
            ($actual: expr, $expected: expr) => {
                let node = MerkleNode::from_hash_list(
                    $actual.into_iter().map(ToOwned::to_owned).collect(),
                );

                assert_eq!(node.hash, $expected);
            };
        }

        assert_hash_eq!(
            ["A", "B", "C", "D", "E", "F", "G", "H"],
            "24A5154C51E5EA8F72DBFCD82F42202F94BB34E1A1D764814D0A0BE012DD866E"
        );

        assert_hash_eq!(
            ["A", "B", "C", "D", "E"],
            "AE4F3A195A3CBD6A3057C205DEF94520930F03F51F73C5A540D8FDAB05163FEF"
        );
    }
}
