use sha2::{digest::generic_array::GenericArray, Digest, Sha256};
use typenum::consts::U32;

#[derive(Debug)]
pub struct Sha256GenericArray(GenericArray<u8, U32>);

impl ToString for Sha256GenericArray {
    fn to_string(&self) -> String {
        format!("{:X}", self.0)
    }
}

pub fn as_sha256(bytes: &[u8]) -> Sha256GenericArray {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Sha256GenericArray(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        let input = ["A", "B", "C", "D"];
        let expected = [
            "559AEAD08264D5795D3909718CDD05ABD49572E84FE55590EEF31A88A08FDFFD",
            "DF7E70E5021544F4834BBEE64A9E3789FEBC4BE81470DF629CAD6DDB03320A5C",
            "6B23C0D5F35D1B11F9B683F0B0A617355DEB11277D91AE091D399C655B87940D",
            "3F39D5C348E5B79D06E842C114E6CC571583BBF44E4B0EBFDA1A01EC05745D43",
        ];

        for (input, expected) in input.into_iter().zip(expected.into_iter()) {
            let actual = as_sha256(input.as_bytes());

            assert_eq!(actual.to_string(), expected);
        }
    }
}
