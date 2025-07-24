use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::{File, OpenOptions, create_dir_all};
use tokio::sync::Mutex;
use std::sync::Arc;
use chrono::Utc;
use std::io::Result;

// Structure pour gérer le serveur de journalisation
struct ServeurJournalisation {
    fichier_log: Arc<Mutex<File>>,
}

impl ServeurJournalisation {
    // Créer un nouveau serveur avec le fichier de log
    async fn nouveau() -> Result<Self> {
        // Créer le dossier logs s'il n'existe pas
        create_dir_all("logs").await?;
        
        // Ouvrir/créer le fichier de log en mode append
        let fichier = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/server.log")
            .await?;
            
        println!("Serveur de journalisation initialisé");
        println!("Fichier de log : logs/server.log");
        
        Ok(ServeurJournalisation {
            fichier_log: Arc::new(Mutex::new(fichier)),
        })
    }
    
    // Enregistrer un message dans le fichier de log
    async fn enregistrer_message(&self, adresse_client: &str, message: &str) -> Result<()> {
        let maintenant = Utc::now();
        let horodatage = maintenant.format("%Y-%m-%d %H:%M:%S UTC");
        
        // Format : [TIMESTAMP] CLIENT_IP: MESSAGE
        let ligne_log = format!("[{}] {}: {}\n", horodatage, adresse_client, message);
        
        // Écrire dans le fichier de façon thread-safe
        let mut fichier = self.fichier_log.lock().await;
        fichier.write_all(ligne_log.as_bytes()).await?;
        
        println!("Message enregistré de {} : {}", adresse_client, message);
        Ok(())
    }
    
    // Gérer une connexion client
    async fn gerer_client(&self, mut stream: TcpStream, adresse: String) {
        println!("Nouvelle connexion : {}", adresse);
        
        // Envoyer un message de bienvenue
        let bienvenue = "Bienvenue sur le serveur de journalisation!\n";
        if stream.write_all(bienvenue.as_bytes()).await.is_err() {
            println!("Erreur lors de l'envoi du message de bienvenue à {}", adresse);
            return;
        }
        
        let mut buffer = [0; 1024];
        
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    // Connexion fermée par le client
                    println!("Client {} s'est déconnecté", adresse);
                    if let Err(e) = self.enregistrer_message(&adresse, "CLIENT_DISCONNECTED").await {
                        eprintln!("Erreur lors de l'enregistrement de la déconnexion : {}", e);
                    }
                    break;
                }
                Ok(n) => {
                    // Convertir les données reçues en string
                    let message_recu = String::from_utf8_lossy(&buffer[..n]);
                    let message_propre = message_recu.trim();
                    
                    if !message_propre.is_empty() {
                        // Enregistrer le message dans le log
                        if let Err(e) = self.enregistrer_message(&adresse, message_propre).await {
                            eprintln!("Erreur lors de l'enregistrement : {}", e);
                        }
                        
                        // Envoyer un accusé de réception au client
                        let reponse = "Message reçu et enregistré\n";
                        if stream.write_all(reponse.as_bytes()).await.is_err() {
                            println!("Erreur lors de l'envoi de la réponse à {}", adresse);
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("Erreur de lecture pour {} : {}", adresse, e);
                    break;
                }
            }
        }
    }
    
    // Démarrer le serveur
    async fn demarrer(self: Arc<Self>, port: u16) -> Result<()> {
        let adresse_serveur = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&adresse_serveur).await?;
        
        println!("Serveur de journalisation démarré sur {}", adresse_serveur);
        println!("En écoute des connexions...");
        
        // Enregistrer le démarrage du serveur
        self.enregistrer_message("SERVER", "Serveur de journalisation démarré").await?;
        
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let adresse_client = addr.to_string();
                    
                    // Enregistrer la nouvelle connexion
                    if let Err(e) = self.enregistrer_message(&adresse_client, "CLIENT_CONNECTED").await {
                        eprintln!("Erreur lors de l'enregistrement de la connexion : {}", e);
                    }
                    
                    // Cloner la référence Arc pour la tâche
                    let serveur_ref = Arc::clone(&self);
                    
                    // Spawner une tâche asynchrone pour chaque client
                    tokio::spawn(async move {
                        serveur_ref.gerer_client(stream, adresse_client).await;
                    });
                }
                Err(e) => {
                    eprintln!("Erreur lors de l'acceptation de connexion : {}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== SERVEUR DE JOURNALISATION ASYNCHRONE ===");
    
    // Créer le serveur de journalisation
    let serveur = match ServeurJournalisation::nouveau().await {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("Erreur lors de l'initialisation du serveur : {}", e);
            return Err(e);
        }
    };
    
    // Démarrer le serveur sur le port 8080
    let port = 8080;
    if let Err(e) = serveur.demarrer(port).await {
        eprintln!("Erreur lors du démarrage du serveur : {}", e);
        return Err(e);
    }
    
    Ok(())
}