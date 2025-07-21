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

    fn renommer(&mut self, nouveau_nom: String) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom,
            solde: self.solde,
        }
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

    let mut compte2 = CompteBancaire {
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

    println!("\nFermeture de tous les comptes:");
    for compte in comptes.into_iter() {
        compte.fermer();
    }
}