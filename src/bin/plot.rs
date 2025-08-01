use std::error::Error;
use std::fs;
use std::fs::File;
use serde::Deserialize;
use plotters::prelude::*;

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

fn load_theta(path: &str) -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let mut parts = content.split_whitespace();
    let theta0 = parts.next().ok_or("No theta0 found")?.parse::<f64>()?;
    let theta1 = parts.next().ok_or("No theta1 found")?.parse::<f64>()?;
    let min = parts.next().ok_or("No min found")?.parse::<f64>()?;
    let max = parts.next().ok_or("No max found")?.parse::<f64>()?;
    Ok((theta0, theta1, min, max))
}

fn normalize(km: f64, min: f64, max: f64) -> f64 {
    if (max - min).abs() < 1e-8 {
        0.0
    } else {
        (km - min) / (max - min)
    }
}

fn r2_score(data: &[DataPoint], theta0: f64, theta1: f64, min: f64, max: f64) -> f64 {
    let mean_price = data.iter().map(|d| d.price).sum::<f64>() / data.len() as f64;
    let ss_tot = data.iter().map(|d| (d.price - mean_price).powi(2)).sum::<f64>();
    let ss_res = data.iter().map(|d| {
        let norm_km = normalize(d.km, min, max);
        d.price - (theta0 + theta1 * norm_km)
    }).map(|e| e.powi(2)).sum::<f64>();
    1.0 - ss_res / ss_tot
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = match read_csv("data/data.csv") {
        Ok(d) if !d.is_empty() => d,
        Ok(_) => {
            eprintln!("Erreur : le fichier data/data.csv est vide.");
            return Ok(());
        },
        Err(e) => {
            eprintln!("Erreur lors de la lecture de data/data.csv : {}", e);
            return Ok(());
        }
    };
    let (theta0, theta1, min, max) = match load_theta("theta.txt") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Erreur lors du chargement de theta.txt : {}", e);
            return Ok(());
        }
    };
    let r2 = r2_score(&data, theta0, theta1, min, max);
    println!("R² score: {:.4}", r2);

    let root = BitMapBackend::new("plot.png", (1400, 900)).into_drawing_area();
    root.fill(&WHITE)?;
    let (min_km, max_km) = data.iter().fold((f64::MAX, f64::MIN), |(min, max), d| (min.min(d.km), max.max(d.km)));
    // Forcer l'axe des prix à [0, 10000]
    let min_price = 0.0;
    let max_price = 10000.0;
    let mut chart = ChartBuilder::on(&root)
        .caption("Régression linéaire : Prix en fonction du kilométrage", ("sans-serif", 40))
        .margin(60)
        .x_label_area_size(70)
        .y_label_area_size(90)
        .build_cartesian_2d(min_km..max_km, min_price..max_price)?;
    chart.configure_mesh()
        .x_desc("Kilométrage (km)")
        .y_desc("Prix (€)")
        .axis_desc_style(("sans-serif", 22))
        .draw()?;
    // Nuage de points
    let scatter = chart.draw_series(
        data.iter().map(|d| Circle::new((d.km, d.price), 5, RED.filled())),
    )?;
    scatter.label("Données (km, prix)").legend(|(x, y)| Circle::new((x, y), 5, RED.filled()));
    // Droite de régression (utilise la normalisation)
    let line = chart.draw_series(LineSeries::new(
        vec![
            (min_km, theta0 + theta1 * normalize(min_km, min, max)),
            (max_km, theta0 + theta1 * normalize(max_km, min, max))
        ],
        &BLUE,
    ))?;
    line.label("Régression linéaire").legend(|(x, y)| PathElement::new(vec![(x-10, y), (x+10, y)], &BLUE));
    // Affichage de la légende
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 20))
        .position(SeriesLabelPosition::LowerRight)
        .draw()?;
    println!("✅ Le graphique a été généré dans plot.png !\n- Rouge : données d'entraînement\n- Bleu : droite de régression\n- R² = {:.4}", r2);
    Ok(())
}
