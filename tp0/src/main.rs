use std::io;

fn main() {

    let _nom = "Kevin";
    let _age:u32 = 30; // u32 = entier non signé sur 32 bits (valeur pos)
    let _age_papa = 70; // rust comprend que c'est un entier par défaut i32
    let _temperature:f32 = 32.5;

    //println!("Hello, world!");
    println!("J'ai {} ans.", _age);
    println!("Papa a {} ans.", _age_papa);
    println!("Il fait {} degrés", _temperature);

    // i32 32 signé     -2xxx à 2xxxxxxx
    // u32 32 non signé 0 à 4xxxxxxx
    // i64 64 signé     très grand intervalle
    // u8  8  non signé 0 à 255

    // 2. Fonction
    let resultat = addition(1, 2);
    println!("La somme est {}", resultat);

    sayHello("xd");

    // conditions et boucles
    let nombre = 16;
    if nombre % 2 == 0{
        println!("Pair");
    } else {
        println!("Impair");
    }

    //boucle for
    for i in 1..=10 {
        println!(" i vaut {}", i);
    }

    for i in 1..6 {
        println!(" i vaut {}", i);
    }

    //itérer sur un tableau
    let voitures = ["jeep", "renault", "bmw"];
    for voiture in voitures {
        println!("Voiture : {}", voiture);
    }

    // for (index, valeur) in collection.iter().enumerate(){}
    // on peut utiliser index et valeur ici }

    // on reprend l'exemple de voiture
    for ( i, voiture) in voitures.iter().enumerate(){
        println!("Index {} : {}", i, voiture);
    }

    // Exemple de vecteur
    let noms = vec![String::from("Kevin"), String::from("Noureddine")];
    for (i, nom) in noms.iter().enumerate(){
        println!("Nom {}: {}", i, nom);
    }

    // Usage de enumerate dans un cas réel : Afficher un Menu avec numéro et choix
    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];
    println!("Menu:");
    for (i, option) in options.iter().enumerate(){
        println!("{}.{}", i+1, option);
    }

    println!("Veuillez saisir un numéro de votre choix:");
    
    let mut choix = String::new(); //gérer sécurité mémoire : variable mutable (modifier une variable sans la déplacer)
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
    } else {
        println!("Vous avez sélectionné : {}", options[choix-1]);
        // ici on peut exécuter une action selon choix dans options
    }

    // Les tableaux
    let tab:[i32;4] = [11, 23, 19, 19];
    let _tab2:[i32;4] = [11, 23, 19, 19]; //pour éviter le warning de variable inutilisée on rajoute le _

    // parcourir le tableau
    for i in 0..tab.len() {
        println!("le tableau tab {}", tab[i]);
    }

    for &elt in &tab {
        println!("l'élément du tableau est {}", elt);
    }
    // &elt => itérer sur des références aux éléments du tableau
    // &tab => on passe une référence au tableau pour éviter de prendre la possession du tableau entier

    //les loop
    let mut compteur = 0;
    loop {
        println!(" Compteur : {}", compteur);
        compteur += 1;
        if compteur == 3 {
            break;
        }
    }

    // boucle while
    let mut compteur2 = 0;
    while compteur2 < 4 {
        println!(" Compteur 2 = {}", compteur2);
        compteur2 += 1;
    }

    // structure
    struct Salarie {
        nom: String,
        ville: String,
        age: u32
    }

    // usage struct => on crée une instance de la structure
    let kevin = Salarie {
        nom: String::from("Kevin"),
        ville: String::from("Lyon"),
        age: 22
    };

    // accès aux attributs de la structure
    println!("Nom : {}, Ville : {}, Age : {}", kevin.nom, kevin.ville, kevin.age);


    // Match
    let nombre = 5;
    match nombre {
        1 => println!("Un"),
        2 => println!("Deux"),
        3 => println!("Trois"),
        4 => println!("Quatre"),
        5 => println!("Cinq"),
        _ => println!("Autre nombre"),
    }

    // Fonctions associées (impl) pour des structures (struct)
    struct Personne {
        nom: String,
    }

    impl Personne {
        fn afficher(&self) { // emprunt immuable => ne modifie rien
            println!("La personne suivante {} est convoquée ", self.nom);
        }
    }

    let personne = Personne {
        nom: "Alexandre".to_string(),
    };

    personne.afficher();

    // Exemple compteur struct
    struct Compteur {
        value: u32,
    }

    impl Compteur {
        fn afficher(&self) { // lecture de la valeur sans la modifier
            println!("Valeur actuelle du compteur : {}", self.value);
        }

        fn incrementer(&mut self) { // mut pour modifier la valeur
            self.value += 1;
        }

        fn deplacer(self){ // prend la possession de self, il n'est plus accessible après. Transfert complet (consommation)
            println!("Déplacé: {}", self.value);
        }
    }

    let mut compteur = Compteur { value: 0 };
    compteur.afficher();
    compteur.incrementer();
    compteur.afficher();
}

fn addition(n1:i32, n2:i32) -> i32 {
    return n1+n2;
}

fn sayHello(nom: &str){
    println!("Bonjour, {}", nom);
}