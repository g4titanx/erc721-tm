use std::fmt;

#[derive(Debug)]
pub enum ERC721Error {
    TokenNotFound,
    NotOwner,
    TransferFailed,
}

impl fmt::Display for ERC721Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ERC721Error::TokenNotFound => write!(f, "Token not found"),
            ERC721Error::NotOwner => write!(f, "Transfer failed: Not the owner"),
            ERC721Error::TransferFailed => write!(f, "Transfer failed"),
        }
    }
}

impl std::error::Error for ERC721Error {}