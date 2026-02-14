use alloc::string::String;
use core::fmt;

/// Error type for witness database operations.
#[derive(Debug)]
pub enum WitnessDbError {
    /// Incomplete or missing witness data.
    TrieWitness(String),
    /// Missing state for a block number.
    StateNotFound(u64),
    /// RLP decoding error.
    Rlp(alloy_rlp::Error),
}

impl fmt::Display for WitnessDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TrieWitness(msg) => write!(f, "trie witness error: {msg}"),
            Self::StateNotFound(num) => write!(f, "state for block {num} not found"),
            Self::Rlp(err) => write!(f, "RLP decode error: {err}"),
        }
    }
}

impl core::error::Error for WitnessDbError {}

impl revm_database_interface::DBErrorMarker for WitnessDbError {}

impl From<alloy_rlp::Error> for WitnessDbError {
    fn from(err: alloy_rlp::Error) -> Self {
        Self::Rlp(err)
    }
}
