use tokio::time::{sleep, Duration};

// créer une fonction asynchrone et futures 
async fn task ( nom:&str, duree:u64)  -> String{

    println!(" Début de la tâche :{}", nom);
    sleep(Duration::from_secs(duree)).await;
    println!("Fin de tâche :{}", nom);
    format!("Resultat de {}", nom)
}

#[tokio::main] // indique que la fonction main est asynchrone 
 async fn main() {

      let  debut = std::time::Instant::now();

    println!("début de mon programme !");
    // je crée une fonction asynchrone qui attend 3 secondes 
      sleep(Duration::from_secs(3)).await;
      println!(" fin du programme après 3 secondes"); 
      let resultat = task("Task1",5).await;
        println!("Résultat 1 reçu : {}", resultat); 
      let resultat2 = task("Task1",5).await;
        println!("Résultat 2 reçu : {}", resultat2); 
      let resultat3 = task("Task1",10).await;
        println!("Résultat 3 reçu : {}", resultat3); 

       println!("Temps total : {:?}", debut.elapsed()); 
    

     // si on veut lancer 3 tâches en parallèle on utilise  join
     // use tokio::join   appel de la bibliothèque avant le main 
     // sinon directement 
     
                  let ( res1, res2, res3 ) = tokio::join!(
                          task("Task1",3),
                          task("Task2",5),
                          task("Task3",3),
                  );
     
      let debut2 = std::time::Instant::now();
      println!("Début des tâches en parallèle !");
     
      println!("Résultat 1 reçu : {}", res1);
      println!("Résultat 2 reçu : {}", res2);

      println!("Résultat 3 reçu : {}", res3);
      println!("Temps total : {:?}", debut2.elapsed());

}
