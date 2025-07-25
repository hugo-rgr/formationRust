use tokio::net::UdpSocket;
use std::io;
use std::time::Duration;

// Structure pour une requête DNS
struct DnsQuery {
    id: u16,
    domain: String,
}

impl DnsQuery {
    fn new(domain: String) -> Self {
        // Générer un ID aléatoire
        let id = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() & 0xFFFF) as u16;
        
        DnsQuery { id, domain }
    }
    
    // Créer un message DNS de requête
    fn to_bytes(&self) -> Vec<u8> {
        let mut request = Vec::new();
        
        // Header DNS (12 bytes)
        // ID (2 bytes)
        request.extend_from_slice(&self.id.to_be_bytes());
        
        // Flags (2 bytes) - QR=0 (requête), RD=1 (recursion desired)
        request.extend_from_slice(&0x0100u16.to_be_bytes());
        
        // Questions count (2 bytes)
        request.extend_from_slice(&1u16.to_be_bytes());
        
        // Answers count (2 bytes) - 0 pour une requête
        request.extend_from_slice(&0u16.to_be_bytes());
        
        // Authority records (2 bytes) - 0
        request.extend_from_slice(&0u16.to_be_bytes());
        
        // Additional records (2 bytes) - 0
        request.extend_from_slice(&0u16.to_be_bytes());
        
        // Question section
        request.extend_from_slice(self.domain.as_bytes());
        request.push(0); // null terminator
        
        request
    }
}

// Parser une réponse DNS
fn parse_dns_response(data: &[u8], query_id: u16) -> Option<String> {
    if data.len() < 12 {
        return None;
    }
    
    // Vérifier l'ID
    let response_id = u16::from_be_bytes([data[0], data[1]]);
    if response_id != query_id {
        println!("ID de réponse incorrect: {} != {}", response_id, query_id);
        return None;
    }
    
    // Vérifier les flags
    let flags = u16::from_be_bytes([data[2], data[3]]);
    let is_response = (flags & 0x8000) != 0;
    let rcode = flags & 0x000F; // Response code
    
    if !is_response {
        println!("Ce n'est pas une réponse DNS");
        return None;
    }
    
    if rcode != 0 {
        println!("Erreur DNS: code {}", rcode);
        return None;
    }
    
    // Nombre de réponses
    let answer_count = u16::from_be_bytes([data[6], data[7]]);
    
    if answer_count == 0 {
        println!("Aucune réponse trouvée");
        return None;
    }
    
    // Chercher l'IP
    // Ici on cherche les 4 derniers bytes comme IP
    if data.len() >= 4 {
        let ip_start = data.len() - 4;
        let ip = format!("{}.{}.{}.{}", 
            data[ip_start], 
            data[ip_start + 1], 
            data[ip_start + 2], 
            data[ip_start + 3]
        );
        Some(ip)
    } else {
        None
    }
}

// Résoudre un domaine avec timeout
async fn resolve_domain(domain: &str, server: &str) -> Result<String, String> {
    println!("Résolution de '{}' via {}", domain, server);
    
    // Créer la requête DNS
    let query = DnsQuery::new(domain.to_string());
    let request_bytes = query.to_bytes();
    
    // Créer le socket UDP
    let socket = UdpSocket::bind("0.0.0.0:0").await
        .map_err(|e| format!("Erreur création socket: {}", e))?;
    
    // Envoyer la requête
    socket.send_to(&request_bytes, server).await
        .map_err(|e| format!("Erreur envoi: {}", e))?;
    
    println!("Requête envoyée ({} bytes)", request_bytes.len());
    
    // Attendre la réponse avec timeout
    let mut buffer = [0; 512];
    
    match tokio::time::timeout(Duration::from_secs(5), socket.recv_from(&mut buffer)).await {
        Ok(Ok((size, from_addr))) => {
            println!("Réponse reçue de {} ({} bytes)", from_addr, size);
            
            // Parser la réponse
            if let Some(ip) = parse_dns_response(&buffer[..size], query.id) {
                Ok(ip)
            } else {
                Err("Impossible de parser la réponse DNS".to_string())
            }
        }
        Ok(Err(e)) => Err(format!("Erreur réception: {}", e)),
        Err(_) => Err("Timeout - aucune réponse reçue".to_string()),
    }
}

#[tokio::main]
async fn main() {
    println!("*** Client DNS ***");
    
    let dns_server = "127.0.0.1:8053";
    
    // Mode interactif
    println!("Tapez un nom de domaine à résoudre (ou 'quit' pour quitter)");
    println!("Domaines disponibles: google.com, localhost, test.local");
    
    loop {
        print!("DNS> ");
        io::Write::flush(&mut io::stdout()).unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erreur de lecture");
            continue;
        }
        
        let domain = input.trim();
        
        if domain.is_empty() {
            continue;
        }
        
        if domain == "quit" {
            println!("Au revoir!");
            break;
        }
        
        // Résoudre le domaine
        match resolve_domain(domain, dns_server).await {
            Ok(ip) => {
                println!("{} -> {}", domain, ip);
            }
            Err(e) => {
                println!("Erreur: {}", e);
            }
        }
        
        println!();
    }
}