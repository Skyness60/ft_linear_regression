use std::error::Error;
use std::fs;
use std::io;

fn load_theta(path: &str) -> Result<(f64, f64), String> {
    let content = fs::read_to_string(path).map_err(|_| format!("Impossible de lire {}. As-tu lancé l'entraînement ?", path))?;
    let mut parts = content.split_whitespace();
    let theta0 = parts.next().ok_or("theta0 manquant dans le fichier theta.txt")?.parse::<f64>().map_err(|_| "theta0 n'est pas un nombre valide".to_string())?;
    let theta1 = parts.next().ok_or("theta1 manquant dans le fichier theta.txt")?.parse::<f64>().map_err(|_| "theta1 n'est pas un nombre valide".to_string())?;
    Ok((theta0, theta1))
}

fn main() {
    println!("Entrez le kilométrage de la voiture :");
    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        eprintln!("Erreur de lecture de l'entrée utilisateur.");
        return;
    }
    let km: f64 = match input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Veuillez entrer un nombre valide pour le kilométrage.");
            return;
        }
    };
    let (theta0, theta1) = match load_theta("theta.txt") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Erreur lors du chargement de theta.txt : {}", e);
            return;
        }
    };
    let price = theta0 + theta1 * km;
    println!("Prix estimé : {:.2} €", price);
}
