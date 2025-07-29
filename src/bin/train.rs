use std::io::Write;
use std::error::Error;
use std::fs::File;
use serde::Deserialize;

struct Theta {
    theta0: f64,
    theta1: f64,
}

#[derive(Deserialize)]
struct DataPoint {
    km: f64,
    price: f64,
}

fn read_csv(path: &str) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let record: DataPoint = result?;
        data.push(record);
    }
    Ok(data)
}

fn gradient_descent(data: &[DataPoint], learning_rate: f64, iterations: usize) -> Theta {
    let m = data.len() as f64;
    let mut theta0 = 0.0;
    let mut theta1 = 0.0;
    for _ in 0..iterations {
        let mut sum_error0 = 0.0;
        let mut sum_error1 = 0.0;
        for point in data {
            let prediction = theta0 + theta1 * point.km;
            let error = prediction - point.price;
            sum_error0 += error;
            sum_error1 += error * point.km;
        }
        theta0 -= learning_rate * (1.0 / m) * sum_error0;
        theta1 -= learning_rate * (1.0 / m) * sum_error1;
    }
    Theta { theta0, theta1 }
}

fn save_theta(theta: &Theta, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    writeln!(file, "{} {}", theta.theta0, theta.theta1)?;
    Ok(())
}

fn main() {
    let data = match read_csv("data/data.csv") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Erreur lors de la lecture du fichier data/data.csv : {}", e);
            return;
        }
    };
    if data.is_empty() {
        eprintln!("Le fichier data/data.csv est vide ou invalide.");
        return;
    }
    let learning_rate = 0.0000000001;
    let iterations = 1000000;
    let theta = gradient_descent(&data, learning_rate, iterations);
    println!("theta0 = {}\ntheta1 = {}", theta.theta0, theta.theta1);
    if let Err(e) = save_theta(&theta, "theta.txt") {
        eprintln!("Erreur lors de l'écriture de theta.txt : {}", e);
    }
}