//use super::block::block::{BlockType}; todo: reuse
use super::crypto::hash::{H256};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum BlockId {
    Hash(H256),
}

impl std::default::Default for BlockId {
    fn default() -> Self { BlockId::Hash(H256::default()) }
}

//impl Clone for BlockId {
//    fn clone(&self) -> BlockId { BlockId::Hash(self); }
//}

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum PropBlockLeaderStatus{
    ConfirmedLeader,
    PotentialLeader,
    NotALeader
}

#[derive(Copy, Clone)]
pub enum VoterBlockStatus{
    OnMainChain,
    Orphan
}

// Todo: Import enum block type from block
#[derive(PartialEq, Copy, Clone)]
pub enum NodeType{
    Transaction,
    Proposer,
    Voter,
}

// Returns the type of the Node
pub trait Node{
    fn get_type() -> NodeType;
}

pub trait Genesis{
    fn get_type() -> Self;
}