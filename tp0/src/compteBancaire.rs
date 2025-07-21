struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire{
    fn afficher(&self) {
        println!("Compte: {}, Solde: {}€", self.nom, self.solde);
    }

    fn deposer(&mut self, montant: f64) {
        self.solde += montant;
        println!("+{} € déposes:", montant);
    }

    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("-{} € retirés:", montant);
        } else {
            println!("Solde insuffisant pour retirer {}€", montant);
        }
    }

    fn fermer(self){ //self ici est consommé ici, on ne peut plus utiliser ensuite
        println!("le compte de {} est fermé, dernier solde : {}€", self.nom, self.solde);
    }
}

fn main(){
    let mut compte = CompteBancaire {
        nom: "Hugo".to_string(),
        solde: 1000.00,
    };

    compte.afficher();
    compte.deposer(200.00);
    compte.afficher();
    compte.retirer(150.00);
    compte.afficher();
    
    compte.fermer();
}