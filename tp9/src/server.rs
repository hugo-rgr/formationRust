use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};

// Liste des clients connectés
type ClientList = Arc<Mutex<Vec<tokio::sync::mpsc::UnboundedSender<Message>>>>;

// Gérer une connexion WebSocket
async fn handle_websocket(
    stream: TcpStream,
    clients: ClientList,
    client_addr: std::net::SocketAddr
) {
    println!("Nouvelle connexion WebSocket: {}", client_addr);
    
    // Effectuer le handshake WebSocket
    let mut ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("Erreur handshake WebSocket: {}", e);
            return;
        }
    };
    
    println!("Handshake WebSocket réussi pour {}", client_addr);
    
    // Canal pour les messages à envoyer à ce client
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Ajouter ce client à la liste
    {
        let mut client_list = clients.lock().unwrap();
        client_list.push(tx.clone());
    }
    
    loop {
        tokio::select! {
            // Recevoir des messages du client WebSocket
            result = ws_stream.next() => {
                match result {
                    Some(Ok(message)) => {
                        match message {
                            Message::Text(text) => {
                                println!("[{}] Message reçu: {}", client_addr, text);
                                
                                // Créer le message de diffusion
                                let broadcast_msg = format!("[{}] {}", client_addr, text);
                                let ws_message = Message::Text(broadcast_msg);
                                
                                // Diffuser à tous les autres clients
                                broadcast_message(&ws_message, &clients, &tx).await;
                            }
                            Message::Binary(data) => {
                                println!("[{}] Message binaire reçu ({} bytes)", client_addr, data.len());
                                
                                // Diffuser les données binaires
                                let ws_message = Message::Binary(data);
                                broadcast_message(&ws_message, &clients, &tx).await;
                            }
                            Message::Close(_) => {
                                println!("[{}] Connexion fermée", client_addr);
                                break;
                            }
                            _ => {}
                        }
                    }
                    Some(Err(e)) => {
                        println!("[{}] Erreur WebSocket: {}", client_addr, e);
                        break;
                    }
                    None => {
                        println!("[{}] Connexion fermée", client_addr);
                        break;
                    }
                }
            }
            
            // Envoyer des messages à ce client
            Some(message) = rx.recv() => {
                if ws_stream.send(message).await.is_err() {
                    break;
                }
            }
        }
    }
    
    // Nettoyer: supprimer ce client de la liste
    remove_client(&tx, &clients).await;
    println!("[{}] Client déconnecté", client_addr);
}

// Diffuser un message à tous les clients (sauf l'expéditeur)
async fn broadcast_message(
    message: &Message,
    clients: &ClientList,
    sender_tx: &tokio::sync::mpsc::UnboundedSender<Message>
) {
    let client_list = clients.lock().unwrap();
    
    for client_tx in client_list.iter() {
        // Ne pas renvoyer le message à l'expéditeur
        if !client_tx.same_channel(sender_tx) {
            let _ = client_tx.send(message.clone());
        }
    }
}

// Supprimer un client de la liste
async fn remove_client(
    client_tx: &tokio::sync::mpsc::UnboundedSender<Message>,
    clients: &ClientList
) {
    let mut client_list = clients.lock().unwrap();
    client_list.retain(|tx| !tx.same_channel(client_tx));
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("*** Serveur WebSocket ***");
    
    // Liste des clients connectés
    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));
    
    // Écouter sur le port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur WebSocket en écoute sur ws://127.0.0.1:8080");
    
    // Accepter les connexions
    while let Ok((stream, addr)) = listener.accept().await {
        let clients_clone = Arc::clone(&clients);
        
        tokio::spawn(async move {
            handle_websocket(stream, clients_clone, addr).await;
        });
    }
    
    Ok(())
}