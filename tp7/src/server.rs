use tokio::net::UdpSocket;
use std::collections::HashMap;

// Structure d'un message DNS
struct DnsMessage {
    id: u16,           // ID de la requête
    is_response: bool, // false=requête, true=réponse
    domain: String,    // Nom de domaine demandé
    ip: Option<String>, // IP de réponse (si trouvée)
}

impl DnsMessage {
    // Parser un message DNS depuis des bytes
    fn parse_request(data: &[u8]) -> Option<DnsMessage> {
        if data.len() < 12 {
            return None; // Trop court pour être un message DNS valide
        }
        
        // Lire l'ID (2 premiers bytes)
        let id = u16::from_be_bytes([data[0], data[1]]);
        
        // Flags (byte 2-3) - on vérifie que c'est une requête
        let flags = u16::from_be_bytes([data[2], data[3]]);
        let is_response = (flags & 0x8000) != 0; // Bit QR
        
        if is_response {
            return None; // On ne traite que les requêtes
        }
        
        // Extraire le nom de domaine (simplification: on prend à partir du byte 12)
        let domain_bytes = &data[12..];
        let domain = String::from_utf8_lossy(domain_bytes).trim_end_matches('\0').to_string();
        
        Some(DnsMessage {
            id,
            is_response: false,
            domain,
            ip: None,
        })
    }
    
    // Créer une réponse DNS
    fn create_response(&self, ip: Option<String>) -> Vec<u8> {
        let mut response = Vec::new();
        
        // Header DNS
        // ID (2 bytes)
        response.extend_from_slice(&self.id.to_be_bytes());
        
        // Flags (2 bytes) - 0x8400 = 1000 0100 0000 0000 en binaire : QR=1 (bit 15 : réponse), AA=1 (bit 10 : authoritative = "je suis autoritaire pour ce domaine")
        let flags: u16 = if ip.is_some() { 0x8400 } else { 0x8403 }; // 0x8403 : NXDOMAIN si pas trouvé
        response.extend_from_slice(&flags.to_be_bytes());
        
        // Questions count - on répète la question originale
        response.extend_from_slice(&1u16.to_be_bytes());
        
        // Answers count - 1 si on a trouvé, 0 sinon
        let answer_count = if ip.is_some() { 1u16 } else { 0u16 };
        response.extend_from_slice(&answer_count.to_be_bytes());
        
        // Authority records (2 bytes) - 0
        response.extend_from_slice(&0u16.to_be_bytes());
        
        // Additional records (2 bytes) - 0
        response.extend_from_slice(&0u16.to_be_bytes());
        
        // On répète exactement la question posée
        response.extend_from_slice(self.domain.as_bytes()); // "google.com"
        response.push(0); // null terminator
        
        // Si on a une réponse, ajouter la section Answer
        if let Some(ip_addr) = ip {
            // Answer section
            response.extend_from_slice(self.domain.as_bytes()); // Répéter le nom de domaine
            response.push(0); // null terminator
            response.extend_from_slice(&1u16.to_be_bytes()); // Type A = "adresse IPv4"
            response.extend_from_slice(&1u16.to_be_bytes()); // Class IN = "Internet"
            response.extend_from_slice(&300u32.to_be_bytes()); // TTL 300 secondes = "garde cette réponse 300 secondes"
            response.extend_from_slice(&4u16.to_be_bytes()); // Data length (4 pour IPv4) = "4 bytes suivent"
            
            // IP address (4 bytes)
            let ip_parts: Vec<u8> = ip_addr.split('.')
                .map(|s| s.parse::<u8>().unwrap_or(0))
                .collect();
            if ip_parts.len() == 4 {
                response.extend_from_slice(&ip_parts); // e.g. [8, 8, 8, 8] pour "8.8.8.8"
            }
        }
        
        response
    }
}

// Base de données DNS
fn create_dns_database() -> HashMap<String, String> {
    let mut db = HashMap::new();
    
    // Domaines prédéfinis
    db.insert("google.com".to_string(), "8.8.8.8".to_string());
    db.insert("localhost".to_string(), "127.0.0.1".to_string());
    db.insert("test.local".to_string(), "192.168.1.100".to_string());
    
    db
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("*** Serveur DNS Simple ***");
    
    // Créer la base de données DNS
    let dns_db = create_dns_database();
    
    println!("Base de données DNS chargée:");
    for (domain, ip) in &dns_db {
        println!("  {} -> {}", domain, ip);
    }
    
    // Créer le socket UDP
    let socket = UdpSocket::bind("127.0.0.1:8053").await?;
    println!("Serveur DNS en écoute sur 127.0.0.1:8053");
    
    let mut buffer = [0; 512]; // Buffer pour les messages DNS
    
    loop {
        // Recevoir une requête DNS
        match socket.recv_from(&mut buffer).await {
            Ok((size, client_addr)) => {
                println!("Requête reçue de {} ({} bytes)", client_addr, size);
                
                // Parser la requête DNS
                if let Some(request) = DnsMessage::parse_request(&buffer[..size]) {
                    println!("Domaine demandé: '{}'", request.domain);
                    
                    // Chercher dans la base de données
                    let ip = dns_db.get(&request.domain).cloned();
                    
                    if let Some(ref found_ip) = ip {
                        println!("Résolution trouvée: {} -> {}", request.domain, found_ip);
                    } else {
                        println!("Domaine non trouvé: {}", request.domain);
                    }
                    
                    // Créer et envoyer la réponse
                    let response = request.create_response(ip);
                    
                    if let Err(e) = socket.send_to(&response, client_addr).await {
                        eprintln!("Erreur lors de l'envoi de la réponse: {}", e);
                    } else {
                        println!("Réponse envoyée à {}", client_addr);
                    }
                } else {
                    println!("Impossible de parser la requête DNS");
                }
                
                println!("---");
            }
            Err(e) => {
                eprintln!("Erreur de réception: {}", e);
            }
        }
    }
}