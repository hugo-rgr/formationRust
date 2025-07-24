# RUST - ROGER Hugo M1 AL
*21/07/2025*

## Présentation de Rust

- Présentation de Rust (langage utilisé pour l'embarqué et le web)
- Beaucoup plus sécurisé que C++ pour applications critiques, notamment en matière de gestion de mémoire

## I. Mise en place de Rust

*Aller sur le site officiel de Rust et le télécharger*

### Vérification de l'installation

Pour vérifier la version de Rust et qu'il soit bien installé :
```bash
rustc --version
```

Pour vérifier Cargo (gestionnaire de paquets de Rust, permet de créer des nouveaux projets, compiler, gérer dépendances) :
```bash
cargo --version
```

Rust est un langage compilé (son compilateur est rustc et son extension est .rs)
Génère un .exe ou un .out après compilation

### Commandes Cargo de base

**Créer un nouveau projet :**
```bash
cargo new tp0
```

**Build le projet :**
```bash
cargo build
```

**Exécuter le projet :**
```bash
cargo run
```

**Exécuter les tests :**
```bash
cargo test
```

**Créer un exécutable puis le lancer :**
```bash
rustc main.rs
./main.exe
```

Les dépendances sont à ajouter dans le fichier `Cargo.toml`

## II. Notions de code de base en Rust

Dans le fichier main.rs, `fn main()` est la fonction principale.

### 1) Variables

`let` pour déclarer une variable

Exemple avec entier non signé sur 32 bits et affichage dans la console via println :

```rust
let age:u32 = 30; // u32 = entier non signé sur 32 bits (valeur positive).
//Si :u32 n'est pas spécifié, par défaut, le compilateur déduit que c'est un i32 quand
//variable est un nombre
println!("J'ai {} ans.", age);
```

Par convention de RUST, il faut utiliser le snake_case, et ne jamais commencer par chiffre, ni espace, ni tirets

### 2) Fonctions

`fn` définit une fonction

**Déclarer une fonction (exemple avec addition) :**

```rust
fn addition(n1:i32, n2:i32) -> i32 { // il faut bien spécifier le type de retour avec ->
    return n1+n2;
}

// Appeler et afficher la fonction dans la fonction main
let resultat = addition(1, 2);
println!("La somme est {}", resultat);
```

**Référence :** `&str` est de type de chaîne de caractère

Exemple :

```rust
fn sayHello(nom: &str){
    println!("Bonjour, {}", nom);
}

// Appeler et afficher la fonction dans la fonction main
sayHello("Loggi");
```

### 3) Conditions et boucles

**Condition if :**

```rust
let nombre = 16;
if nombre % 2 == 0 {
    println!("Pair");
} else {
    println!("Impair");
}
```

**Boucle for :**

```rust
for i in 1..=10 {
    // intervalle inclusif (ici 10 inclus)
    println!(" i vaut {}", i);
}

for i in 1..6 {
    // intervalle exclusif (ici 6 est exclu)
    println!(" i vaut {}", i);
}

let voitures = ["jeep", "renault", "bmw"];

for voiture in voitures {
    //itérer sur un tableau
    println!("Voiture : {}", voiture);
}

for (i, voiture) in voitures.iter().enumerate() {
    // créer un itérateur sur la collection avec iter() 
    // et créer une séquence de index,valeur avec enumerate()
    println!("Index {} : {}", i, voiture);
}
```

**Exemple de vecteur :** un vecteur, contrairement à un tableau classique, à sa taille qui croît ou diminue de manière dynamique en fonction des besoins

```rust
let noms = vec![String::from("Kevin"), String::from("Noureddine")];
for (i, nom) in noms.iter().enumerate(){
    println!("Nom {}: {}", i, nom);
}
```

**match : pattern-matching :**

```rust
let mut choix = String::new();
let choix:usize = match choix.trim().parse(){
    Ok(num) => num,
    Err(_) => {
        println!("Veuillez saisir un numéro de valide");
        return;
    }
};

let nombre = 5;
match nombre {
    1 => println!("Un"),
    2 => println!("Deux"),
    3 => println!("Trois"),
    4 => println!("Quatre"),
    5 => println!("Cinq"),
    _ => println!("Autre nombre"), //cas par défaut
}
```

**boucles loop :**

```rust
loop {
    println!(" Compteur : {}", compteur);
    compteur += 1;
    if compteur == 3 {
        break;
    }
}
```

**boucles while :**

```rust
let mut compteur2 = 0;
while compteur2 < 4 {
    println!(" Compteur 2 = {}", compteur2);
    compteur2 += 1;
}
```

### 4) Les tableaux

On peut parcourir les tableaux avec des indices :

```rust
let tab:[i32;4] = [11, 23, 19, 19];

for i in 0..tab.len() {
    println!("le tableau tab {}", tab[i]);
}
```

On peut aussi parcourir par référencement :

```rust
for &elt in &tab {
    println!("l'élément du tableau est {}", elt);
}
```

### 5) Structures

**Définition de la structure :**

```rust
struct Salarie {
    nom: String,
    ville: String,
    age: u32
}
```

**Usage (création d'une instance de la structure) :**

```rust
let kevin = Salarie {
    nom: String::from("Kevin"),
    ville: String::from("Lyon"),
    age: 22
};
```

**Accès aux attributs :**

```rust
println!("Nom : {}, Ville : {}, Age : {}", kevin.nom, kevin.ville, kevin.age);
```

**impl :** pour implémenter des fonctions associées à la structure portant le même nom :

```rust
impl Personne { // associé à la structure Personne
    fn afficher(&self) {
        println!("La personne suivante {} est convoquée ", self.nom);
    }
}
```

## 4) Notions diverses

### Input/output

```rust
// au début du fichier : import de libraries
use std::io;
```

### Variable mutable (modifier une variable sans la déplacer) :

```rust
let mut choix = String::new();
```

### self (en paramètres de fonction) :

- **&self :** lecture de la valeur sans la modifier
- **&mut self :** pour rendre la valeur modifiable dans la fonction
- **self :** prend la possession de self, il n'est plus accessible après. Transfert complet (consommation). Utile pour des types qui ne doivent être utilisés qu'une seule fois (ex: sockets, tokens)

## II. Entrées / sorties

Il faut bien mettre la gestion d'erreurs dans la fonction main :

```rust
fn main() ->io::Result<()> {// gestion d'erreurs
    Ok(()) // exemple de valeur à retourner dans le main (ou Err(e) s'il y a erreur)
}
```

### 1) Ecriture dans un fichier

Il faut bien importer ces libraries :

```rust
use std::fs::File;
use std::io::{self, Write};
```

**Créer un fichier :**
```rust
File::create();
```

**Ecrire dans un fichier :**
```rust
file_write_all();
```

### 2) Lecture à partir d'un fichier

**Importer ces libraries :**

```rust
use std::fs::File;
use std::io::{self,BufReader,Read};
```

**Ouvrir fichier :**
```rust
File::open();
```

**Lecteur tamponné :**
```rust
BufReader::new(file); // file = fichier ouvert
// il a des fonctions comme read_to_string() pour afficher 
// le contenu du fichier sous forme en string
```

## III. TPs

### 1) TP1

Implémentation d'un système de gestion de compte bancaire simple avec l'interpréteur de Rust.

**On peut :**
- Afficher le solde
- Déposer de l'argent
- Voir le compte
- Quitter le programme

Le but était d'appliquer les concepts de base de Rust, ainsi que de mettre en place une gestion d'entrée utilisateur.

### 2) TP2

Amélioration du système de gestion de compte bancaire du TP1 avec utilisation d'une structure et implémentations de fonctions. Il gère plusieurs comptes bancaires qu'on met dans un vecteur (liste dynamique).

**On peut :**
- Créer un nouveau compte
- Afficher le solde du compte choisi
- Déposer de l'argent sur le compte choisi
- Retirer de l'argent sur le compte choisi
- Renommer un compte choisi
- Afficher la liste de tous les comptes
- Fermer un compte choisi
- Quitter le programme

Le but était aussi de manipuler les structures et les concepts d'ownership pour la gestion mémoire.

---
---

*24/07/2025*

## I) Dates

* Bibliothèque chrono::Utc (importer en dépendance dans le fichier Cargo.toml) :

```rust
chrono = {version="<version>", features=[<feature choisie 1>, <feature choisie 2>, <etc>]}
```

* Exemple:

```rust
Utc::now(); //heure actuelle
Utc::now().format("%d/%m/%Y %H:%M:%S") // formatter date au format français jj/mm/AAAA HH/MM/SS 
```

## II) Ownerships / memberships

* Owner = propriétaire
* Membership = appartenance à une structure
⇒ Pourquoi ? Pour garantir la sécurité mémoire

Ownership et borrowing sont des concepts clés de Rust pour la gestion de la mémoire.
* 1 valeur = 1 propriété unique, responsable de libérer la mémoire lorsqu'elle sort du scope
* Le propriétaire peut être transféré : Quand le propriétaire est déplacé, l'ancien propriétaire ne peut plus y accéder
* En sortant du scope, la valeur est libérée automatiquement

```rust
let prenom = String::from("Nourddine"); // prenom possède le String
let prenom_ref = &prenom; // emprunt immuable, prenom garde l'ownership
println!("Le prénom est : {}", prenom_ref);

let prenom_clone = prenom.clone(); // copie indépendante, prenom garde l'ownership
let prenom2 = prenom; // MOVE : prenom n'est plus accessible, prenom2 devient propriétaire

// let prenom3 = prenom.clone(); // ERREUR : prenom a été moved, plus accessible
```

Cela peut être appliqué à des fonctions aussi, grâce à leurs paramètres :

```rust
fn prendre_ownership(nom: String){}
//lecture seule
fn emprunter_lecture(nom: &String) {}
//lecture et écriture
fn emprunter_ecriture(nom: &mut String) {}
fn creer_et_retourner_ownership() -> String {
    let nouveau_nom = String::from("Alice");
    nouveau_nom // ownership transféré vers l'appelant
}
```

* Peut être étendu aux structures :

```rust
let user = User {
  nom : String::from("Alexandre"),
  secu: String::from("1825678290 55")
};
    
println!("nom {}",user.nom);
// display(&user); // &emprumter un champ d'une structure
display(user); 
```

## III) TP3

Création d'un gestionnaire de fichiers interactif utilisant les structures, l'ownership et la gestion des dates.

**On peut :**
* Choisir/Initialiser un fichier (crée le fichier s'il n'existe pas)
* Afficher les informations du fichier (nom et date de sélection)
* Lire le contenu du fichier
* Écrire/Modifier le fichier (en écrasant le contenu existant)
* Ajouter du contenu au fichier
* Supprimer définitivement le fichier
* Quitter le programme

Le but était d'appliquer les concepts d'ownership (notamment avec la méthode de suppression qui consomme `self`), la gestion des fichiers avec les I/O, l'utilisation de la bibliothèque `chrono` pour les dates, et la manipulation des structures avec `impl`. Le programme utilise `match` pour la gestion des erreurs et des options, ainsi que les emprunts avec `&` pour éviter les transferts d'ownership inutiles.

## IV) Threads

- Mutex : Pour protéger l’accès à une donnée partagée entre plusieurs threads.
- Arc: pour partager entre les tâches (comme un pointeur intelligent qui permet de partager la propriété entre plusieurs tâches) :

```rust
Arc::new(Mutex::new(()));
```

- thread::spawn : créer un nouveau thread

## V) Réseau - Serveur asynchrone

- Utilisation de la librarie tokio pour gérer l’asynchrone
- TcpStream::connect("<ip>:<port>") pour flux de données bidirectionnelles
- TcpListener pour connexions entrantes
- Mots clés pour l’asynchrone : async et await
- tokio a aussi des méthodes de thread adaptées à l’asynchrone comme tokio::spawn, tokio::AsyncReadExt, tokio::AsyncReadExt

## VI) TP4

Création d’un serveur de journalisation asynchrone. Ecoute un port TCP, puis reçoit des messages des clients connectés (qui peuvent être connectés simultanément)

Un fichier de log avec horodatage est enregistré.

- Utilisation de tokio et de chrono::Utc
- TCP ( TcpListener et TcpStream)
- Mutex / Arc

Deux projets :
- serveur : `dossier tp4`
- client : `tp4_client`
  
Le client envoie la chaîne de caractères au serveur, et le serveur le reçoit et l’écrit dans le fichier de logs.
Ces deux projets s'exécutent avec un simple `cargo run`, et on peut faire exécuter plusieurs instances du client pour se connecter au serveur.
