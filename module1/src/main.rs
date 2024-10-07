// Importing modules to read from a csv
use std::fs::File;
use std::io::{self, BufReader};
use std::error::Error;
use csv::{ReaderBuilder, StringRecord};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Movie {
    film: String,
    genre: String,
    lead_studio: String,
    audience_score: u32,
    profitability: f64,
    rotten_tomatoes: u32,
    worldwide_gross: String,
    year: u32,
}

fn main() {
    println!("Welcome to the Movie Database Management program.");

    let mut input_num = String::new();

    while input_num.trim() != "3" {
        println!();
        println!("1. Print data");
        println!("2. Search data");
        println!("3. Quit");
        println!("Please select an option: ");

        input_num.clear();
        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to read line.");

        if input_num.trim() == "1" {
            // if user input is 1 then run print all function
            if let Err(e) = print_all("movies.csv") {
                println!("Error: {}", e);
            }
        } else if input_num.trim() == "2" {
            // if user input is 2 then run search movie function
            println!();
            println!("Enter the title of the movie you want to search for:");
            let mut title = String::new();
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line.");
            let title = title.trim();

            // Run search movie to find result
            match search_movie("movies.csv", title) {
                Ok(Some(movie)) => println!(),
                Ok(None) => println!(),
                Err(e) => println!("Error: {}", e),
            }
        } else if input_num.trim() == "3" {
            function3(input_num.trim().to_string());
        } else {
            println!("Invalid option. Please enter 1, 2, or 3.");
        }
    }
}

fn print_all(file_name: &str) -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // Specify that the first row contains headers
        .from_reader(reader);

    // Print headers
    let headers = rdr.headers()?;
    println!("Headers: {:?}", headers);

    // Iterate through all the movies and print them
    for result in rdr.records() {
        let record: StringRecord = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn search_movie(file_name: &str, title: &str) -> Result<Option<Movie>, Box<dyn Error>> {
    match search_movie_by_title(file_name, title) {
        Ok(Some(movie)) => {
            println!();
            println!("Film: {}", movie.film);
            println!("Genre: {}", movie.genre);
            println!("Lead Studio: {}", movie.lead_studio);
            println!("Audience Score: {}", movie.audience_score);
            println!("Profitability: {}", movie.profitability);
            println!("Rotten Tomatoes: {}", movie.rotten_tomatoes);
            println!("Worldwide Gross: {}", movie.worldwide_gross);
            println!("Year: {}", movie.year);

            Ok(Some(movie))
        }
        Ok(None) => {
            println!("No movie found with title '{}'", title);
            Ok(None)
        }
        Err(err) => {
            println!("Error searching for movie: {}", err);
            Err(err)
        }
    }
}


fn search_movie_by_title(file_name: &str, title: &str) -> Result<Option<Movie>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.records() {
        let record = result?;

        // Attempt to deserialize the record into a Movie instance
        let movie: Movie = record.deserialize(None)?;

        if movie.film.eq_ignore_ascii_case(title) {
            return Ok(Some(movie));
        }
    }
    Ok(None)
}

fn function3(num: String) {
    println!();
    println!("You have entered {}. Exiting program.", num);
}
