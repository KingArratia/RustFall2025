use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::error::Error;

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).expect("Unable to create file");

    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year)
            .expect("Unable to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    for line in reader.lines() {
        if let Ok(entry) = line {
            let parts: Vec<&str> = entry.split(',').collect();
            if parts.len() == 3 {
                let title = parts[0].to_string();
                let author = parts[1].to_string();
                let year: u16 = parts[2].trim().parse().unwrap_or(0);
                books.push(Book { title, author, year });
            }
        }
    }

    books
}

fn main() -> Result<(), Box<dyn Error>> {
    let books = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
            year: 1949,
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            year: 1960,
        },
        Book {
            title: "The Catcher in the Rye".to_string(),
            author: "J.D. Salinger".to_string(),
            year: 1951,
        },
    ];

    // Save books to a file
    save_books(&books, "books.txt");
    println!("Books saved to file.");

    // Load them back
    let loaded_books = load_books("books.txt");
    println!("\nLoaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }

    Ok(())
}
