use books_description_parser::*;
use pest::Parser;
use std::env;
use std::fs;
use std::process;

fn help() {
    println!("This is a CLI tool to parse and display book data.");
    println!("Usage:");
    println!("  parse <file_path>   Parse the given book description file");
    println!("  credits             Display credits information");
}

fn credits() {
    println!("Credits:");
    println!("  Developed by [Melnyk Danyil]");
}

fn parse_file(file_path: &str) -> Result<String, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;
    let cleaned_content = content.replace("\r", "");
    Ok(cleaned_content)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Missing command. Run `help` for usage instructions.");
        process::exit(1);
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                println!("Error: Missing file path. Usage: parse <file_path>");
                process::exit(1);
            }
            let file_path = &args[2];
            match parse_file(file_path) {
                Ok(input) => {
                    // Parse the input using the grammar
                    let parsed = Grammar::parse(Rule::book, &input).expect("Failed to parse");
                    let book_pair = parsed.into_iter().next().expect("No book found");

                    let book = Book::from_pair(book_pair);

                    println!("{:#?}", book);

                    let json_output = serde_json::to_string_pretty(&book)
                        .expect("Failed to serialize book to JSON");
                    println!("{}", json_output);
                }
                Err(e) => {
                    println!("{}", e);
                    process::exit(1);
                }
            }
        }
        "credits" => {
            credits();
        }
        "help" => {
            help();
        }
        _ => {
            println!(
                "Error: Unknown command '{}'. Run `help` for usage instructions.",
                args[1]
            );
            process::exit(1);
        }
    }
}
