use std::fs::{File, remove_file};
use std::io::{self, BufReader, Read, Write};
use chrono::Utc;

struct Fichier {
    nom_fichier: String,
    date_creation: String,
}

impl Fichier {
    // Choisir un fichier existant ou l'initialiser s'il n'existe pas
    fn choisir_ou_initialiser(nom: String) -> Self {
        let maintenant = Utc::now();
        
        // Vérifier si le fichier existe déjà en essayant de l'ouvrir
        match File::open(&nom) {
            Ok(_) => {
                println!("Fichier '{}' choisi!", nom);
            }
            Err(_) => {
                // Le fichier n'existe pas, on l'initialise
                match File::create(&nom) {
                    Ok(_) => println!("Fichier '{}' initialisé avec succès!", nom),
                    Err(e) => println!("Erreur lors de l'initialisation du fichier: {}", e),
                }
            }
        }
        
        Fichier {
            nom_fichier: nom,
            date_creation: maintenant.format("%d/%m/%Y %H:%M:%S").to_string(),
        }
    }

    // Afficher les informations du fichier
    fn afficher_info(&self) {
        println!("Fichier: {} | Sélectionné le: {}", self.nom_fichier, self.date_creation);
    }

    // Lire le contenu d'un fichier
    fn lire_fichier(&self) {
        match File::open(&self.nom_fichier) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut contenu = String::new();
                match reader.read_to_string(&mut contenu) {
                    Ok(_) => {
                        println!("*** Contenu du fichier '{}' ***", self.nom_fichier);
                        println!("{}", contenu);
                        println!("*** Fin du contenu ***");
                    }
                    Err(e) => println!("Erreur lors de la lecture: {}", e),
                }
            }
            Err(e) => println!("Impossible d'ouvrir le fichier: {}", e),
        }
    }

    // Écrire dans un fichier (créer ou écraser)
    fn ecrire_fichier(&self, contenu: &str) {
        match File::create(&self.nom_fichier) {
            Ok(mut file) => {
                match file.write_all(contenu.as_bytes()) {
                    Ok(_) => println!("Fichier '{}' créé/modifié avec succès!", self.nom_fichier),
                    Err(e) => println!("Erreur lors de l'écriture: {}", e),
                }
            }
            Err(e) => println!("Impossible de créer le fichier: {}", e),
        }
    }

    // Ajouter du contenu à un fichier existant
    fn ajouter_contenu(&self, contenu: &str) {
        // D'abord lire le contenu existant
        let mut contenu_existant = String::new();
        match File::open(&self.nom_fichier) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let _ = reader.read_to_string(&mut contenu_existant);
            }
            Err(_) => {}, // Si le fichier n'existe pas, on continue avec un contenu vide
        }
        
        // Ajouter le nouveau contenu
        contenu_existant.push_str(&contenu);
        
        // Réécrire tout le fichier
        match File::create(&self.nom_fichier) {
            Ok(mut file) => {
                match file.write_all(contenu_existant.as_bytes()) {
                    Ok(_) => println!("Contenu ajouté au fichier '{}'", self.nom_fichier),
                    Err(e) => println!("Erreur lors de l'ajout: {}", e),
                }
            }
            Err(e) => println!("Impossible d'ouvrir le fichier: {}", e),
        }
    }

    // Supprimer définitivement un fichier
    fn supprimer_fichier(self) {
        match remove_file(&self.nom_fichier) {
            Ok(_) => println!("Fichier '{}' supprimé définitivement!", self.nom_fichier),
            Err(e) => println!("Erreur lors de la suppression: {}", e),
        }
        // self est consommé ici, le fichier n'est plus utilisable
    }
}

fn main() -> io::Result<()> {
    let mut fichier: Option<Fichier> = None;
    
    loop {
        afficher_menu();
        
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        
        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un numéro valide!");
                continue;
            }
        };
        
        match choix {
            1 => {
                println!("Nom du fichier à choisir/initialiser:");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur de lecture");
                let nom = nom.trim().to_string();
                
                fichier = Some(Fichier::choisir_ou_initialiser(nom));
                println!("Opération terminée!");
            }
            
            2 => {
                match &fichier {
                    Some(f) => f.afficher_info(),
                    None => println!("Aucun fichier choisi!"),
                }
            }
            
            3 => {
                match &fichier {
                    Some(f) => f.lire_fichier(),
                    None => println!("Aucun fichier choisi!"),
                }
            }
            
            4 => {
                match &fichier {
                    Some(f) => {
                        println!("Contenu à écrire:");
                        let mut contenu = String::new();
                        io::stdin().read_line(&mut contenu).expect("Erreur de lecture");
                        f.ecrire_fichier(contenu.trim());
                    }
                    None => println!("Aucun fichier choisi!"),
                }
            }
            
            5 => {
                match &fichier {
                    Some(f) => {
                        println!("Contenu à ajouter:");
                        let mut contenu = String::new();
                        io::stdin().read_line(&mut contenu).expect("Erreur de lecture");
                        f.ajouter_contenu(contenu.trim());
                    }
                    None => println!("Aucun fichier choisi!"),
                }
            }
            
            6 => {
                match fichier.is_some() {
                    true => {
                        println!("Êtes-vous sûr de vouloir supprimer le fichier? (oui/non)");
                        let mut confirmation = String::new();
                        io::stdin().read_line(&mut confirmation).expect("Erreur de lecture");
                        
                        if confirmation.trim().to_lowercase() == "oui" {
                            match fichier.take() {
                                Some(f) => f.supprimer_fichier(),
                                None => {},
                            }
                        } else {
                            println!("Suppression annulée.");
                        }
                    }
                    false => println!("Aucun fichier choisi!"),
                }
            }
            
            7 => {
                break;
            }
            
            _ => println!("Option invalide! Choisissez entre 1 et 7."),
        }
        
        println!("\nAppuyez sur Entrée pour continuer...");
        let mut pause = String::new();
        let _ = io::stdin().read_line(&mut pause);
    }
    
    Ok(())
}

fn afficher_menu() {
    let options = [
        "Choisir/Initialiser un fichier",
        "Afficher informations du fichier", 
        "Lire le fichier",
        "Écrire/Modifier le fichier (en écrasant le contenu existant)",
        "Ajouter du contenu au fichier",
        "Supprimer définitivement le fichier",
        "Quitter"
    ];
    
    println!("\n*** MENU ***");
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    println!("Votre choix:");
}