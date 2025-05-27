use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
pub index: u64,
pub previous_hash: String,
pub timestamp: String,
pub data: String,
pub hash: String,
pub signature: String,
}

impl Block {
pub fn new(index: u64, previous_hash: String, data: String, signature: String) -> Self {
let timestamp = Utc::now().to_rfc3339();
let content = format!("{}{}{}{}", index, &previous_hash, &timestamp, &data);
let hash = format!("{:x}", Sha256::digest(content.as_bytes()));

Block {  
        index,  
        previous_hash,  
        timestamp,  
        data,  
        hash,  
        signature,  
    }  
}

}

