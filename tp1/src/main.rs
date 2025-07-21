use std::io;

fn main() {
    let numero_compte: &str = "001";
    let titulaire: &str = "John Doe";
    let solde: f64 = 1000.00; //€

    let comptes_list = vec![numero_compte];

    //let numero_compte2: &str = "002";
    //let titulaire2: &str = "John Smith";
    //let solde2: f64 = 569.36; //€

    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];
    println!("Menu:");
    for (i, option) in options.iter().enumerate(){
        println!("{}.{}", i+1, option);
    }

    println!("Veuillez saisir un numéro de votre choix:");
    
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
         1 => afficher_solde(solde),
         2 => effectuer_retrait(solde),
         3 => lister_comptes(&comptes_list),
         4 => {
             println!("Merci d'avoir utilisé notre système bancaire. Au revoir !");
             return;
         },
         _ => unreachable!(),
    }
}

fn afficher_solde(solde: f64) {
    println!("SOLDE : {}€", solde);
}

fn effectuer_retrait(solde: f64) {
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

    if montant > solde {
        println!("Solde insuffisant ! Solde actuel : {}€", solde);
        return;
    }

    let new_solde = solde - montant;
    println!("Retiré : {}€, Nouveau solde : {}€", montant, new_solde);
}

fn lister_comptes(comptes_list: &Vec<&str>) {
    println!("Liste des comptes:");
    for (i, compte) in comptes_list.iter().enumerate(){
        println!("Compte {}: {}", i + 1, compte);
    }
}