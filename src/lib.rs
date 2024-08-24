mod error;
mod proof;
use std::collections::HashMap;
use error::ERC721Error;

#[derive(Clone, Debug)]
pub struct ERC721Token {
    pub id: u64,
    pub owner: String,
    pub metadata: String,
}

impl ERC721Token {
    fn new(id: u64, owner: String, metadata: String) -> Self {
        Self { id, owner, metadata }
    }
}

pub struct ERC721 {
    tokens: HashMap<u64, ERC721Token>,
    balances: HashMap<String, u64>,
}

impl ERC721 {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            balances: HashMap::new(),
        }
    }

    pub fn mint(&mut self, to: String, token_id: u64, metadata: String) {
        let token = ERC721Token::new(token_id, to.clone(), metadata);
        self.tokens.insert(token_id, token);
        *self.balances.entry(to).or_insert(0) += 1;
    }

    pub fn transfer(&mut self, from: String, to: String, token_id: u64) -> Result<(), ERC721Error> {
        if let Some(token) = self.tokens.get_mut(&token_id) {
            if token.owner != from {
                return Err(ERC721Error::NotOwner);
            }
            token.owner = to.clone();
            *self.balances.entry(from.clone()).or_insert(1) -= 1;
            *self.balances.entry(to).or_insert(0) += 1;
            Ok(())
        } else {
            Err(ERC721Error::TokenNotFound)
        }
    }

    pub fn owner_of(&self, token_id: u64) -> Result<&String, ERC721Error> {
        self.tokens.get(&token_id).map(|token| &token.owner).ok_or(ERC721Error::TokenNotFound)
    }

    pub fn balance_of(&self, owner: &String) -> u64 {
        *self.balances.get(owner).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minting() {
        let mut erc721 = ERC721::new();
        erc721.mint("Alice".to_string(), 1, "Token1".to_string());

        assert_eq!(erc721.balance_of(&"Alice".to_string()), 1);
        assert_eq!(erc721.owner_of(1).unwrap(), "Alice");
    }

    #[test]
    fn test_transfer() {
        let mut erc721 = ERC721::new();
        erc721.mint("Alice".to_string(), 1, "Token1".to_string());

        assert_eq!(erc721.transfer("Alice".to_string(), "Bob".to_string(), 1), Ok(()));
        assert_eq!(erc721.balance_of(&"Alice".to_string()), 0);
        assert_eq!(erc721.balance_of(&"Bob".to_string()), 1);
        assert_eq!(erc721.owner_of(1).unwrap(), "Bob");
    }

    #[test]
    fn test_transfer_fail() {
        let mut erc721 = ERC721::new();
        erc721.mint("Alice".to_string(), 1, "Token1".to_string());

        let result = erc721.transfer("Bob".to_string(), "Charlie".to_string(), 1);
        assert_eq!(result.unwrap_err().to_string(), "Transfer failed: Not the owner");
    }

    #[test]
    fn test_proof_generation() {
        let erc721 = ERC721::new();
        let result = crate::proof::generate_proof(&erc721, 1);
        assert!(result.is_err());
    }
}
