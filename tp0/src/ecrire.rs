use std::fs::File;
use std::io::{self, Write};

fn main() ->io::Result<()>  {


    let mut file = File::create("test.txt")?; // créer ou écraser un fichier
    file.write_all(b"Bonjour a tous, fichier cree !" )?; // écrit des données dans le fichier
    // b est un byte string on utilise lorsque on travaille avec des données binaires 
    println!("le fichier a été crée avec succes!!!");

      Ok(()) // signifie que que tout s'est bien passé et le () signifie rien à retourner
      Err(e) // signifie qu'une erreur i/o s'est produite  
     // io::Result<()>  retourne un résultat de type 


}