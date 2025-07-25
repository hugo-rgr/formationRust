mod chat_message;

use chat_message::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};

// État global du serveur - liste des clients connectés
type ClientList = Arc<Mutex<Vec<tokio::sync::mpsc::UnboundedSender<ChatMessage>>>>;

// Gérer un client connecté
async fn handle_client(
    mut stream: TcpStream,
    clients: ClientList,
    client_addr: std::net::SocketAddr
) {
    println!("Client connecté: {}", client_addr);
    
    // Canal pour envoyer des messages à ce client
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel(); // canal de communication asynchrone entre tâches
    
    // Ajouter ce client à la liste
    {
        let mut client_list = clients.lock().unwrap();
        client_list.push(tx.clone());
    }
    
    let mut buffer = [0; 1024];
    
    loop {
        tokio::select! {
            // Recevoir des messages du client
            result = stream.read(&mut buffer) => {
                match result {
                    Ok(0) => break, // Client déconnecté
                    Ok(n) => {
                        // Parser le message reçu
                        if let Ok(message) = ChatMessage::from_bytes(&buffer[..n]) {
                            println!("[{}] {}: {}", message.timestamp, message.username, message.content);
                            
                            // Diffuser le message à tous les autres clients
                            broadcast_message(&message, &clients).await;
                        }
                    }
                    Err(_) => break,
                }
            }
            
            // Recevoir des messages à envoyer à ce client
            Some(message) = rx.recv() => {
                let msg_bytes = message.to_bytes();
                if stream.write_all(&msg_bytes).await.is_err() {
                    break;
                }
            }
        }
    }
    
    // Nettoyer à la déconnexion
    remove_client(&tx, &clients).await;
    println!("Client {} déconnecté", client_addr);
}

// Diffuser un message à tous les clients connectés
async fn broadcast_message(message: &ChatMessage, clients: &ClientList) {
    let client_list = clients.lock().unwrap();
    
    for client_tx in client_list.iter() {
        let _ = client_tx.send(message.clone());
    }
}

// Supprimer un client de la liste
async fn remove_client(
    client_tx: &tokio::sync::mpsc::UnboundedSender<ChatMessage>,
    clients: &ClientList
) {
    let mut client_list = clients.lock().unwrap();
    client_list.retain(|tx| !tx.same_channel(client_tx));
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("=== Serveur Chat Simple ===");
    
    // Liste des clients connectés
    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));
    
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur en écoute sur 127.0.0.1:8080");
    
    loop {
        let (stream, addr) = listener.accept().await?;
        let clients_clone = Arc::clone(&clients);
        
        tokio::spawn(async move {
            handle_client(stream, clients_clone, addr).await;
        });
    }
}