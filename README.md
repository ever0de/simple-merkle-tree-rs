# Simple `MerkleTree` implementation using Rust

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
