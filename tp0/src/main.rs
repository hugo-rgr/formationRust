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
        // ici on peut exécuter un
    }

}

fn addition(n1:i32, n2:i32) -> i32 {
    return n1+n2;
}

fn sayHello(nom: &str){
    println!("Bonjour, {}", nom);
}