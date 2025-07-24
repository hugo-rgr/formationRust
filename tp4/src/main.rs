use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use chrono::Utc;

// Fonction pour écrire dans le fichier log
fn write_log(message: &str, client_addr: std::net::SocketAddr) {
    // Créer le dossier logs s'il n'existe pas
    create_dir_all("logs").unwrap();
    
    // Ouvrir le fichier en mode append
    let mut file = OpenOptions::new() // OpenOptions permet de configurer comment ouvrir le fichier
        .create(true) // Créer le fichier s'il n'existe pas
        .append(true) // Ajouter à la fin du fichier s'il existe
        .open("logs/server.log") // Ouvrir le fichier logs/server.log
        .unwrap(); // Gérer l'erreur si le fichier ne peut pas être ouvert
    
    // Créer le timestamp
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    let log_entry = format!("[{}] Client {}: {}\n", timestamp, client_addr, message.trim());
    
    // Écrire dans le fichier
    file.write_all(log_entry.as_bytes()).unwrap();
    println!("Log: {}", log_entry.trim());
}

// Gérer un client connecté
async fn handle_client(mut stream: TcpStream, logger: Arc<Mutex<()>>, client_addr: std::net::SocketAddr) {
    let mut buffer = [0; 1024];
    
    loop {
        // Lire les données du client
        match tokio::io::AsyncReadExt::read(&mut stream, &mut buffer).await {
            Ok(0) => {
                println!("Client déconnecté: {}", client_addr);
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]); // from_utf8_lossy permet de convertir les octets en chaîne de caractères, même si ce n'est pas une chaîne UTF-8 valide.
                
                // Utiliser le mutex pour écrire dans le log de façon thread-safe
                {
                    let _lock = logger.lock().unwrap(); // Acquérir le verrou pour protéger l'accès au fichier log
                    write_log(&message, client_addr);
                }
                
                // Répondre au client
                let response = "Message recu et enregistre\n";
                tokio::io::AsyncWriteExt::write_all(&mut stream, response.as_bytes()).await.unwrap();
            }
            Err(_) => {
                println!("Client déconnecté: {}", client_addr);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Démarrage du serveur de journalisation...");
    
    // Créer un mutex vide pour synchroniser l'accès au fichier
    // Mutex pour protéger l'accès au fichier log, car plusieurs clients peuvent écrire en même temps,
    // donc il faut s'assurer que l'accès est thread-safe, c'est-à-dire qu'un seul client peut écrire à la fois.
    // Comme ceci, on évite les conflits d'écriture dans le fichier log.
    // Arc pour le partager entre les tâches (comme un pointeur intelligent qui permet de partager la propriété entre plusieurs tâches).
    let logger = Arc::new(Mutex::new(()));

    println!("Serveur de journalisation démarré");
    
    // Créer le listener TCP
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap(); // await car lier une adresse peut prendre du temps.
    println!("Serveur en écoute sur 127.0.0.1:7878");
    
    // Boucle principale pour accepter les connexions
    loop {
        let (stream, addr) = listener.accept().await.unwrap(); // Accepter une nouvelle connexion, await car accepter une connexion peut prendre du temps.
        println!("Nouveau client connecté: {}", addr);
        
        let logger_clone = Arc::clone(&logger); // Cloner l'Arc pour partager le mutex avec chaque client.
        
        // Spawner une tâche pour chaque client
        tokio::spawn(handle_client(stream, logger_clone, addr));
    }
}