use pest::iterators::Pair;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    book_title: String,
    authors: Vec<String>,
    genres: Vec<String>,
    publication_year: u16,
    rating: f32,
    price: String,
}

impl Book {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let mut book_title = String::new();
        let mut authors = Vec::new();
        let mut genres = Vec::new();
        let mut publication_year = 0;
        let mut rating = 0.0;
        let mut price = "0 UAH".to_string();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::book_title => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    inner.next();
                    if let Some(title) = inner.next() {
                        book_title = title.as_str().trim_matches('"').to_string();
                    }
                }
                Rule::list_of_authors => {
                    authors = inner_pair
                        .into_inner()
                        .filter_map(|author| {
                            let name = author.as_str().trim();
                            if !name.is_empty() {
                                Some(name.to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                }
                Rule::list_of_genres => {
                    genres = inner_pair
                        .into_inner()
                        .filter_map(|genre| {
                            let genre_name = genre.as_str().trim();
                            if !genre_name.is_empty() {
                                Some(genre_name.to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                }
                Rule::publication_year => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    if let Ok(year) = inner.as_str().trim().parse::<u16>() {
                        publication_year = year;
                    }
                }
                Rule::rating => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    if let Ok(rating_value) = inner.as_str().trim().parse::<f32>() {
                        rating = rating_value;
                    }
                }
                Rule::price => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();
                    if let Some(price_value) = inner.next() {
                        price = price_value.as_str().trim().to_string();
                    }
                    inner.next();
                    if let Some(currency) = inner.next() {
                        price.push_str(&format!(" {}", currency.as_str().trim()));
                    }
                }
                _ => {}
            }
        }

        Book {
            book_title,
            authors,
            genres,
            publication_year,
            rating,
            price,
        }
    }
}
