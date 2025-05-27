use serde::{Serialize, Deserialize};
use serde_json;
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};

/// A block in the blockchain, parameterized over the data payload.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block<T>
where
    T: Serialize + Clone + std::fmt::Debug,
{
    /// Position of the block in the chain
    pub index: u64,
    /// Hash of the previous block
    pub previous_hash: String,
    /// Timestamp in UTC when the block was created
    pub timestamp: DateTime<Utc>,
    /// Payload data stored in the block
    pub data: T,
    /// SHA-256 hash of this block
    pub hash: String,
    /// Digital signature of the block's contents
    pub signature: String,
}

impl<T> Block<T>
where
    T: Serialize + Clone + std::fmt::Debug,
{
    /// Creates a new Block, computing its hash automatically.
    ///
    /// # Arguments
    ///
    /// * `index` - Position of the block in the chain.
    /// * `previous_hash` - Hash of the preceding block.
    /// * `data` - Payload data implementing Serialize + Debug.
    /// * `signature` - Digital signature over the block's contents.
    pub fn new(index: u64, previous_hash: String, data: T, signature: String) -> Self {
        let timestamp = Utc::now();
        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            data,
            hash: String::new(),
            signature,
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Calculates the SHA-256 hash of the block's core fields.
    ///
    /// Serializes the tuple (index, previous_hash, timestamp, data) to JSON
    /// to ensure deterministic ordering.
    pub fn calculate_hash(&self) -> String {
        let content = serde_json::to_string(&(
            &self.index,
            &self.previous_hash,
            &self.timestamp.to_rfc3339(),
            &self.data,
        ))
        .expect("Failed to serialize block content for hashing");

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verifies that the stored hash matches the block's calculated hash.
    pub fn is_valid_hash(&self) -> bool {
        self.hash == self.calculate_hash()
    }

    /// Stub for digital signature verification. Integrate with a crypto library as needed.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key in appropriate format for signature verification.
    pub fn verify_signature(&self, public_key: &str) -> bool {
        // TODO: integrate with a digital signature crate (e.g., ed25519-dalek)
        unimplemented!("Signature verification not implemented");
    }
}
