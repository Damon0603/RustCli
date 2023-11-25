use clap::{App, Arg};
use csv::Reader;
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let matches = App::new("RustCliAnalysis")
        .version("0.1.0")
        .author("Your Name")
        .about("Performs basic epidemiological analysis and can generate sample data")
        .arg(Arg::with_name("file")
                 .help("The CSV file to process")
                 .required_unless("generate-data")
                 .index(1))
        .arg(Arg::with_name("generate-data")
                 .help("Generate sample data")
                 .long("generate-data")
                 .takes_value(false))
        .get_matches();

    if matches.is_present("generate-data") {
        match generate_csv("data.csv", 100) {
            Ok(()) => println!("Sample data generated in 'data.csv'."),
            Err(e) => eprintln!("Failed to generate data: {}", e),
        }
    } else {
        let file_path = matches.value_of("file").unwrap();
        match read_csv(file_path) {
            Ok(data) => {
                println!("Data: {:?}", data);
                let mean = calculate_mean(&data);
                println!("Mean: {}", mean);
                // Add other statistical analyses here
            },
            Err(e) => println!("Error reading CSV: {}", e),
        }
    }
}

fn generate_csv(file_path: &str, num_data_points: usize) -> Result<(), Box<dyn Error>> {
    let mut file = BufWriter::new(File::create(file_path)?);
    let mut rng = rand::thread_rng();

    for _ in 0..num_data_points {
        let value: f64 = rng.gen_range(0.0..100.0);
        writeln!(file, "{:.2}", value)?;
    }

    Ok(())
}

fn read_csv(file_path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if let Ok(value) = record[0].parse::<f64>() {
            data.push(value);
        }
    }

    Ok(data)
}

fn calculate_mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

// You can add more functions for other statistical calculations here, like median, standard deviation, etc.
