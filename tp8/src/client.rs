mod chat_message;

use chat_message::*;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("=== Client Chat Simple ===");
    
    // Demander le pseudo
    print!("Entrez votre pseudo: ");
    io::stdout().flush().unwrap();
    
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();
    
    // Connexion au serveur
    let mut stream = match TcpStream::connect("127.0.0.1:8080").await {
        Ok(s) => s,
        Err(e) => {
            println!("❌ Impossible de se connecter: {}", e);
            return;
        }
    };
    
    println!("✅ Connecté au serveur! Tapez vos messages (ou 'quit' pour quitter):");
    
    // Canal pour l'input utilisateur
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    
    // Thread pour lire l'input utilisateur
    let username_clone = username.clone();
    tokio::spawn(async move {
        loop {
            print!("{}: ", username_clone);
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
    
    let mut buffer = [0; 1024];
    
    loop {
        tokio::select! {
            // Recevoir des messages du serveur
            result = stream.read(&mut buffer) => {
                match result {
                    Ok(0) => {
                        println!("Connexion fermée par le serveur");
                        break;
                    }
                    Ok(n) => {
                        if let Ok(message) = ChatMessage::from_bytes(&buffer[..n]) {
                            // Afficher le message (sauf les siens)
                            if message.username != username {
                                println!("\r[{}] {}: {}", message.timestamp, message.username, message.content);
                                print!("{}: ", username);
                                io::stdout().flush().unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        println!("Erreur lecture: {}", e);
                        break;
                    }
                }
            }
            
            // Traiter l'input utilisateur
            Some(input) = rx.recv() => {
                let input = input.trim();
                
                if input == "quit" {
                    println!("Au revoir!");
                    break;
                }
                
                if input.is_empty() {
                    continue;
                }
                
                // Créer et envoyer le message
                let message = ChatMessage::new(username.clone(), input.to_string());
                let msg_bytes = message.to_bytes();
                
                if stream.write_all(&msg_bytes).await.is_err() {
                    println!("Erreur envoi message");
                    break;
                }
            }
        }
    }
}