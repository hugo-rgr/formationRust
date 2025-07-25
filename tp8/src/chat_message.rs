use serde::{Deserialize, Serialize};

// Structure d'un message de chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub username: String,
    pub content: String,
    pub timestamp: String,   // Heure d'envoi
}

impl ChatMessage {
    // Créer un nouveau message
    pub fn new(username: String, content: String) -> Self {
        let timestamp = chrono::Utc::now().format("%H:%M:%S").to_string();
        ChatMessage {
            username,
            content,
            timestamp,
        }
    }
    
    // Sérialiser en JSON
    pub fn to_bytes(&self) -> Vec<u8> {
        let json = serde_json::to_string(self).unwrap();
        json.as_bytes().to_vec()
    }
    
    // Désérialiser depuis bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let json_str = std::str::from_utf8(data)
            .map_err(|e| format!("Erreur UTF-8: {}", e))?;
        
        serde_json::from_str(json_str)
            .map_err(|e| format!("Erreur JSON: {}", e))
    }
}