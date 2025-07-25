use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("*** Client WebSocket ***");
    
    // Se connecter au serveur WebSocket
    let url = "ws://127.0.0.1:8080";
    
    let (ws_stream, _) = match connect_async(url).await {
        Ok((stream, response)) => {
            println!("Connecté au serveur WebSocket!");
            println!("Statut de la réponse: {}", response.status());
            (stream, response)
        }
        Err(e) => {
            println!("Erreur de connexion: {}", e);
            return;
        }
    };
    
    // Séparer en sink (envoi) et stream (réception)
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
    // Canal pour l'input utilisateur
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    
    // Tâche pour lire l'input utilisateur
    tokio::spawn(async move {
        println!("Tapez vos messages (ou 'quit' pour quitter, 'binary' pour envoyer des données binaires):");
        
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }
            
            if tx.send(input).is_err() {
                break;
            }
        }
    });
    
    loop {
        tokio::select! {
            // Recevoir des messages du serveur
            result = ws_receiver.next() => {
                match result {
                    Some(Ok(message)) => {
                        match message {
                            Message::Text(text) => {
                                println!("\r{}", text);
                                print!("> ");
                                io::stdout().flush().unwrap();
                            }
                            Message::Binary(data) => {
                                println!("\rDonnées binaires reçues ({} bytes): {:?}", 
                                    data.len(), 
                                    &data[..std::cmp::min(10, data.len())] // Afficher les 10 premiers bytes
                                );
                                print!("> ");
                                io::stdout().flush().unwrap();
                            }
                            Message::Close(_) => {
                                println!("Connexion fermée par le serveur");
                                break;
                            }
                            _ => {}
                        }
                    }
                    Some(Err(e)) => {
                        println!("Erreur WebSocket: {}", e);
                        break;
                    }
                    None => {
                        println!("Connexion fermée");
                        break;
                    }
                }
            }
            
            // Traiter l'input utilisateur
            Some(input) = rx.recv() => {
                let input = input.trim();
                
                if input == "quit" {
                    println!("Fermeture de la connexion...");
                    if ws_sender.send(Message::Close(None)).await.is_err() {
                        println!("Erreur fermeture");
                    }
                    break;
                }
                
                if input == "binary" {
                    // Envoyer des données binaires d'exemple
                    let binary_data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello" en bytes
                    println!("Envoi de données binaires: {:?}", binary_data);
                    
                    if ws_sender.send(Message::Binary(binary_data)).await.is_err() {
                        println!("Erreur envoi données binaires");
                        break;
                    }
                    continue;
                }
                
                if input.is_empty() {
                    continue;
                }
                
                // Envoyer un message texte
                if ws_sender.send(Message::Text(input.to_string())).await.is_err() {
                    println!("Erreur envoi message");
                    break;
                }
            }
        }
    }
    
    println!("Client WebSocket fermé");
}