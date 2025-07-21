use std::io;

struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire{
    fn afficher(&self) {
        println!("Compte: {}, Solde: {}€", self.nom, self.solde);
    }

    fn deposer(&mut self, montant: f64) {
        if montant < 0.0 {
            println!("Impossible de déposer un montant négatif: {}€", montant);
            return;
        }
        self.solde += montant;
        println!("+{} € déposés", montant);
    }

    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("-{} € retirés:", montant);
        } else {
            println!("Solde insuffisant pour retirer {}€", montant);
        }
    }

    fn renommer(&mut self, nouveau_nom: String) {
        self.nom = nouveau_nom;
        println!("Compte renommé en: {}", self.nom);
    }

    fn fermer(self){ //self ici est consommé ici, on ne peut plus utiliser ensuite
        println!("le compte de {} est fermé, dernier solde : {}€", self.nom, self.solde);
    }
}

fn main(){
    
    let mut compte1 = CompteBancaire {
        nom: "Hugo".to_string(),
        solde: 1000.00,
    };
    compte1.deposer(200.00);
    compte1.retirer(150.00);
    compte1.afficher();

    // Test dépôt négatif
    compte1.deposer(-50.0);

    // Test renommer
    compte1.renommer("Hugo Dupont".to_string());

    let compte2 = CompteBancaire {  // Supprimé mut inutile
        nom: "John Doe".to_string(),
        solde: 1000.00,
    };
    compte2.afficher();

    // Gestion de plusieurs comptes
    let mut comptes = vec![
        compte1,
        compte2
    ];

    println!("\nTous les comptes:");
    for (index, compte) in comptes.iter().enumerate() {
        print!("{}: ", index + 1);
        compte.afficher();
    }

    println!("\nDépôt de 100€ sur tous les comptes:");
    for compte in comptes.iter_mut() {
        compte.deposer(100.0);
    }

    //println!("\nFermeture de tous les comptes:");
    //for compte in comptes.into_iter() {
    //    compte.fermer();
    //}

    loop {

        let options = ["Créer un compte", "Afficher solde", "Déposer", "Retrait", "Renommer", "Liste comptes", "Fermer un compte", "Quitter"];
        println!("Menu:");
        for (i, option) in options.iter().enumerate(){
            println!("{}.{}", i+1, option);
        }

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Attention erreur de lecture");
        let choix:usize = match choix.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un numéro de valide");
                return;
            }
        };

        if choix == 0 || choix > options.len(){
            println!(" choix hors système !! limite système ");
            return;
        } else {
            println!("Vous avez sélectionné : {}", options[choix-1]);
        }

        match choix {
            1 => {
                println!("Veuillez saisir le nom du nouveau compte:");
                let mut nom_compte = String::new();
                io::stdin().read_line(&mut nom_compte).expect("Erreur lors de la lecture");
                let nom_compte = nom_compte.trim().to_string();
                
                // Vérifier si le compte existe déjà
                if comptes.iter().any(|c| c.nom == nom_compte) {
                    println!("Un compte avec ce nom existe déjà.");
                } else {
                    let nouveau_compte = CompteBancaire {
                        nom: nom_compte,
                        solde: 0.0,
                    };
                    comptes.push(nouveau_compte);
                    println!("Le compte a bien été créé.");
                }
            },
            2 => {
                let nom_compte = saisir_nom_compte();
                // Afficher le solde du compte
                if let Some(compte) = comptes.iter().find(|c| c.nom == nom_compte) {
                    compte.afficher();
                } else {
                    println!("Compte non trouvé.");
                }
            },
            3 => {
                let nom_compte = saisir_nom_compte();
                // Déposer de l'argent sur le compte
                if let Some(compte) = comptes.iter_mut().find(|c| c.nom == nom_compte) {
                    println!("Montant à déposer:");
                    let mut montant_str = String::new();
                    io::stdin().read_line(&mut montant_str).expect("Erreur de lecture");
                    let montant = match montant_str.trim().parse::<f64>() {
                        Ok(montant) => montant,
                        Err(_) => {
                            println!("Montant invalide !");
                            return;
                        }
                    };
                    compte.deposer(montant);
                } else {
                    println!("Compte non trouvé.");
                }
            },
            4 => {
                let nom_compte = saisir_nom_compte();
                // Retirer de l'argent du compte
                if let Some(compte) = comptes.iter_mut().find(|c| c.nom == nom_compte) {
                    println!("Montant à retirer:");
                    let mut montant_str = String::new();
                    io::stdin().read_line(&mut montant_str).expect("Erreur de lecture");
                    let montant = match montant_str.trim().parse::<f64>() {
                        Ok(montant) => montant,
                        Err(_) => {
                            println!("Montant invalide !");
                            return;
                        }
                    };
                    compte.retirer(montant);
                } else {
                    println!("Compte non trouvé.");
                }
            },
            5 => {
                let nom_compte = saisir_nom_compte();
                // Renommer le compte
                if let Some(compte) = comptes.iter_mut().find(|c| c.nom == nom_compte) {
                    println!("Veuillez saisir le nouveau nom du compte:");
                    let mut nouveau_nom = String::new();
                    io::stdin().read_line(&mut nouveau_nom).expect("Erreur de lecture");
                    let nouveau_nom = nouveau_nom.trim().to_string();
                    compte.renommer(nouveau_nom);
                } else {
                    println!("Compte non trouvé.");
                }
            }
            6 => {
                // Afficher la liste de tous les comptes
                println!("Liste des comptes:");
                for compte in &comptes {
                    compte.afficher();
                }
            },
            7 => {
                let nom_compte = saisir_nom_compte();
                // Fermer un compte
                if let Some(index) = comptes.iter().position(|c| c.nom == nom_compte) {
                    let compte = comptes.remove(index);
                    compte.fermer();
                } else {
                    println!("Compte non trouvé.");
                }
            },
            8 => {
                // Quitter le programme
                println!("Merci d'avoir utilisé notre système bancaire. Au revoir !");
                return;
            },
            _ => unreachable!(),
        }
    }

}

fn saisir_nom_compte() -> String {
    println!("Veuillez saisir le nom du compte:");
    let mut nom_compte = String::new();
    io::stdin().read_line(&mut nom_compte).expect("Erreur de lecture");
    nom_compte.trim().to_string()
}