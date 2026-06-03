use crate::Result;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};

pub type DidId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub did: DidId,
    pub public_key: Vec<u8>,
    pub label: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip)]
    secret_key: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedAction {
    pub action_id: String,
    pub actor_did: DidId,
    pub action_type: String,
    pub payload_hash: String,
    pub signature: Vec<u8>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct IdentityManager;

impl IdentityManager {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_identity(label: &str) -> Result<AgentIdentity> {
        let mut sk_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut sk_bytes);
        let signing_key = SigningKey::from_bytes(&sk_bytes);
        let verifying_key = signing_key.verifying_key();

        let did = format!("did:nexus:{}", hex::encode(verifying_key.as_bytes()));

        Ok(AgentIdentity {
            did,
            public_key: verifying_key.as_bytes().to_vec(),
            secret_key: Some(signing_key.to_bytes().to_vec()),
            label: label.to_string(),
            created_at: chrono::Utc::now(),
        })
    }

    pub fn sign_action(identity: &AgentIdentity, action_type: &str, payload: &[u8]) -> Result<SignedAction> {
        use sha2::{Digest, Sha256};
        use uuid::Uuid;

        let payload_hash = format!("sha256:{}", hex::encode(Sha256::digest(payload)));

        let signature = if let Some(ref sk_bytes) = identity.secret_key {
            let sk = SigningKey::from_bytes(sk_bytes.as_slice().try_into().map_err(|_| {
                crate::NexusError::Identity("Invalid secret key".to_string())
            })?);
            let sig: Signature = sk.sign(payload);
            sig.to_bytes().to_vec()
        } else {
            return Err(crate::NexusError::Identity(
                "Cannot sign: secret key not available".to_string(),
            ));
        };

        Ok(SignedAction {
            action_id: Uuid::new_v4().to_string(),
            actor_did: identity.did.clone(),
            action_type: action_type.to_string(),
            payload_hash,
            signature,
            timestamp: chrono::Utc::now(),
        })
    }

    pub fn verify_signature(action: &SignedAction) -> Result<bool> {
        let did = &action.actor_did;
        let pub_key_hex = did.strip_prefix("did:nexus:").unwrap_or("");
        let pub_key_bytes = hex::decode(pub_key_hex).map_err(|_| {
            crate::NexusError::Identity("Invalid DID format".to_string())
        })?;

        let verifying_key = VerifyingKey::from_bytes(
            pub_key_bytes.as_slice().try_into().map_err(|_| {
                crate::NexusError::Identity("Invalid public key".to_string())
            })?,
        ).map_err(|_| crate::NexusError::Identity("Invalid public key bytes".to_string()))?;

        let sig = Signature::from_bytes(
            action.signature.as_slice().try_into().map_err(|_| {
                crate::NexusError::Identity("Invalid signature".to_string())
            })?,
        );

        let payload_hash = action.payload_hash.trim_start_matches("sha256:");
        let payload_bytes = hex::decode(payload_hash).map_err(|_| {
            crate::NexusError::Identity("Invalid payload hash".to_string())
        })?;

        Ok(verifying_key.verify_strict(&payload_bytes, &sig).is_ok())
    }

    pub fn to_did_key(public_key: &[u8]) -> Result<String> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(public_key);
        Ok(format!("did:key:z{}", encoded))
    }
}

impl AgentIdentity {
    pub fn sign(&self, payload: &[u8]) -> Result<SignedAction> {
        IdentityManager::sign_action(self, "generic", payload)
    }
}
