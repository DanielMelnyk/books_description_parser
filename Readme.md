# books_description_parser

## Books Description Parser
`books_description_parser` is a parser designed to extract information from a structured markdown-like catalog of books. The input consists of book descriptions that include details such as book title, authors, genres, publication year, rating, and price.

The parser processes these descriptions and converts them into a structured data format like JSON or Rust structs, which can be used for further analysis, storage, or integration into book-related applications.

## Parsing Process
The parser reads a markdown-like format with structured information for each book. Each book entry contains the following fields:

1. **Book Title**: The title of the book.
2. **Authors**: A list of authors of the book.
3. **Genres**: A list of genres associated with the book.
4. **Publication Year**: The year the book was published.
5. **Rating**: The overall rating of the book.
6. **Price**: The price of the book.

### Grammar
The parser leverages the Pest library to handle the input format. The grammar rules defined in `grammar.pest` process various fields, including strings, numbers, and lists (e.g., authors and genres), ensuring accurate extraction.

```
WHITESPACE = { " " | "\t" }
SPACE = { WHITESPACE+ }
NEWLINE = { "\n" }

book_title = { "Book " ~ book_num ~ ":" ~ SPACE? ~ quoted_text ~ NEWLINE }
publication_year = { "Publication Year:" ~ SPACE? ~ ASCII_DIGIT+ ~ NEWLINE }
price = { "Price:" ~ SPACE? ~ number ~ SPACE? ~ "UAH" ~ NEWLINE }

book_num = { ASCII_DIGIT+ }
number = { ("-"? ~ ASCII_DIGIT+) ~ (("." ~ ASCII_DIGIT+)?) }
quoted_text = { "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
```

### Example of Input
```markdown
Book 1: "Enemy Of My Enemy"
Authors: [Travis Casey, Melissa Mayberry]
Genres: [Fiction, Thriller, Drama]
Publication Year: 2016
Rating: 9.5
Price: 199.00 UAH
```

### Example of Output
```json
{
  "book_title": "Enemy Of My Enemy",
  "authors": [
    "Travis Casey",
    "Melissa Mayberry"
  ],
  "genres": [
    "Fiction",
    "Thriller",
    "Drama"
  ],
  "publication_year": 2016,
  "rating": 9.5,
  "price": 199.00
}
```