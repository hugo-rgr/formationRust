// lecture à partir d'un fichier 
// pour lire un fichier on doit ouvrir le fichier et lire son contenu
// dans notre cas on on utilise Read et BufReader
// BufReader => crée un lecteur tamponé pour améliorer la performance

use std::fs::File; // stream  file system
use std::io::{self,BufReader,Read};
use std::process::Command;

fn main() -> io::Result<()>{

       
        let file = File::open("test.txt")?;
        let mut reader = BufReader::new(file);// on crée un lecteur tamponné
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        println!("Ceci est le contenu du fichier :\n {}", content);

        // Result::Ok(());
        // Result::Err("une erreur s'est produite!!!!!"); 

        //let mut choix = String::new();
        //let _= io::stdin().read_line(&mut choix);

        let _ = Command::new("pause").status(); 

    Ok(())
}