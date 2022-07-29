# Simple `MerkleTree` implementation using Rust

[![Coverage Status](https://coveralls.io/repos/github/ever0de/simple-merkle-tree-rs/badge.svg)](https://coveralls.io/github/ever0de/simple-merkle-tree-rs)

## Disable `hash` features

### Merkle tree structure without hashing

```rust
MerkleTree {
    root: MerkleNode {
        hash: "ABCDE",
        left: Some(
            MerkleNode {
                hash: "ABCD",
                left: Some(
                    MerkleNode {
                        hash: "AB",
                        left: Some(
                            MerkleNode {
                                hash: "A",
                                left: None,
                                right: None,
                            },
                        ),
                        right: Some(
                            MerkleNode {
                                hash: "B",
                                left: None,
                                right: None,
                            },
                        ),
                    },
                ),
                right: Some(
                    MerkleNode {
                        hash: "CD",
                        left: Some(
                            MerkleNode {
                                hash: "C",
                                left: None,
                                right: None,
                            },
                        ),
                        right: Some(
                            MerkleNode {
                                hash: "D",
                                left: None,
                                right: None,
                            },
                        ),
                    },
                ),
            },
        ),
        right: Some(
            MerkleNode {
                hash: "E",
                left: None,
                right: None,
            },
        ),
    },
}
```
