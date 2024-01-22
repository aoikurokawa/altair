use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieAccountState {
    pub discriminator: String,
    pub is_initialized: bool,
    pub reviewer: Pubkey,
    pub rating: u8,
    pub title: String,
    pub description: String,
}

impl MovieAccountState {
    pub const DISCRIMINATOR: &'static str = "review";

    pub fn get_account_size(title: &str, description: &str) -> usize {
        return (4 + Self::DISCRIMINATOR.len())
            + 1
            + 1
            + 32
            + (4 + title.len())
            + (4 + description.len());
    }
}

impl Sealed for MovieAccountState {}

impl IsInitialized for MovieAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieCommentCounter {
    pub discriminator: String,
    pub is_initialized: bool,
    pub counter: u64,
}

impl MovieCommentCounter {
    pub const DISCRIMINATOR: &'static str = "counter";

    pub const SIZE: usize = (4 + Self::DISCRIMINATOR.len()) + 1 + 8;
}

impl Sealed for MovieCommentCounter {}

impl IsInitialized for MovieCommentCounter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieComment {
    pub discriminator: String,
    pub is_initialized: bool,
    pub review: Pubkey,
    pub commenter: Pubkey,
    pub comment: String,
    pub count: u64,
}

impl MovieComment {
    pub const DISCRIMINATOR: &'static str = "comment";

    pub fn get_account_size(comment: &str) -> usize {
        return (4 + Self::DISCRIMINATOR.len()) + 1 + 32 + 32 + (4 + comment.len()) + 8;
    }
}

impl Sealed for MovieComment {}

impl IsInitialized for MovieComment {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
