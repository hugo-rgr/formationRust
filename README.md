# RUST - ROGER Hugo M1 AL

# 21/07/2025

## Présentation de Rust

- Présentation de Rust (langage utilisé pour l'embarqué et le web)
- Beaucoup plus sécurisé que C++ pour applications critiques, notamment en matière de gestion de mémoire

## I. Mise en place de Rust

Aller sur le site officiel de Rust et le télécharger.

### Vérification de l'installation

Pour vérifier la version de Rust et qu'il soit bien installé :
```bash
rustc --version
```

Pour vérifier Cargo (gestionnaire de paquets de Rust, permet de créer des nouveaux projets, compiler, gérer dépendances) :
```bash
cargo --version
```

Rust est un langage compilé (son compilateur est rustc et son extension est .rs). Il génère un .exe ou un .out après compilation.

### Créer un nouveau projet

```bash
cargo new tp0
```

### Build le projet

```bash
cargo build
```

### Exécuter le projet

```bash
cargo run
```

### Exécuter les tests

```bash
cargo test
```

### Créer un exécutable puis le lancer

```bash
rustc main.rs
./main.exe
```

**Note :** Les dépendances sont à ajouter dans le fichier `Cargo.toml`

## II. Notions de code de base en Rust

Dans le fichier `main.rs`, `fn main()` est la fonction principale.

### 1) Variables

`let` pour déclarer une variable

**Exemple avec entier non signé sur 32 bits et affichage dans la console via println :**

```rust
let age: u32 = 30; // u32 = entier non signé sur 32 bits (valeur positive).
// Si :u32 n'est pas spécifié, par défaut, le compilateur déduit que c'est un i32 quand la variable est un nombre
println!("J'ai {} ans.", age);
```

**Par convention de RUST :** il faut utiliser le snake_case, et ne jamais commencer par chiffre, ni espace, ni tirets

### 2) Fonctions

`fn` définit une fonction

**Déclarer une fonction (exemple avec addition) :**

```rust
fn addition(n1: i32, n2: i32) -> i32 { // il faut bien spécifier le type de retour avec ->
    return n1 + n2;
}

// Appeler et afficher la fonction dans la fonction main
let resultat = addition(1, 2);
println!("La somme est {}", resultat);
```

**Référence :** `&str` est de type de chaîne de caractère

**Exemple :**

```rust
fn sayHello(nom: &str) {
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
    // itérer sur un tableau
    println!("Voiture : {}", voiture);
}

for (i, voiture) in voitures.iter().enumerate() {
    // créer un itérateur sur la collection avec iter() 
    // et créer une séquence de index,valeur avec enumerate()
    println!("Index {} : {}", i, voiture);
}
```

**Exemple de vecteur :** un vecteur, contrairement à un tableau classique, a sa taille qui croît ou diminue de manière dynamique en fonction des besoins

```rust
let noms = vec![String::from("Kevin"), String::from("Noureddine")];
for (i, nom) in noms.iter().enumerate() {
    println!("Nom {}: {}", i, nom);
}
```

**match : pattern-matching :**

```rust
let mut choix = String::new();
let choix: usize = match choix.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Veuillez saisir un numéro de valide");
        return;
    }
};
```

### 4) Notions diverses

**Input/output**

```rust
// au début du fichier : import de libraries
use std::io;
```

**Variable mutable (modifier une variable sans la déplacer) :**

```rust
let mut choix = String::new();
```
