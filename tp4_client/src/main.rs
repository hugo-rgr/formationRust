use tokio::net::TcpStream;
use std::io;

// Fonction asynchrone pour envoyer un message
async fn send_message(message: &str) -> String {
    println!("Début de l'envoi: {}", message);
    
    // Se connecter au serveur
    let mut stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();
    
    // Envoyer le message
    tokio::io::AsyncWriteExt::write_all(&mut stream, message.as_bytes()).await.unwrap();
    
    println!("Fin de l'envoi: {}", message.trim());
    format!("Message envoyé: {}", message.trim())
}

#[tokio::main]
async fn main() {
    println!("Client de journalisation démarré");
    
    // Proposer le choix du mode
    println!("Choisissez le mode:");
    println!("1. Mode normal (interactif)");
    println!("2. Mode parallèle (tâches automatiques)");
    println!("Tapez 1 ou 2:");
    
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    
    match choix.trim() {
        "1" => {
            // Mode normal - interactif
            println!("\n--- Mode normal (interactif) ---");
            let mut stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();
            println!("Connecté au serveur de journalisation!");
            
            loop {
                println!("Tapez votre message (ou 'quit' pour quitter):");
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                
                if input.trim() == "quit" {
                    println!("Déconnexion...");
                    break;
                }
                
                tokio::io::AsyncWriteExt::write_all(&mut stream, input.as_bytes()).await.unwrap();
                println!("Message envoyé: {}", input.trim());
            }
        }
        "2" => {
            // Mode parallèle - tâches automatiques
            println!("\n--- Mode parallèle (tâches automatiques) ---");
            println!("Lancement de plusieurs tâches en parallèle...");
            let debut = std::time::Instant::now();
            
            // Lancer plusieurs tâches en parallèle indépendantes
            let t1 = tokio::spawn(send_message("log de A\n"));
            let t2 = tokio::spawn(send_message("log de B\n"));
            let t3 = tokio::spawn(send_message("log de C\n"));
            
            // Attendre que toutes les tâches se terminent
            let (res1, res2, res3) = tokio::join!(t1, t2, t3);
            
            println!("Résultats:");
            println!("- {}", res1.unwrap());
            println!("- {}", res2.unwrap());
            println!("- {}", res3.unwrap());
            
            println!("Toutes les tâches terminées avec succès en {:?}", debut.elapsed());
        }
        _ => {
            println!("Choix invalide. Au revoir!");
        }
    }
}