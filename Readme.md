# books_description_parser

## Books Description Parser
`books_description_parser` is a parser designed to extract information from a structured markdown-like catalog of books. The input consists of book descriptions that include details such as book title, authors, genres, publication year, rating, and price.

The parser processes these descriptions and converts them into a structured data format like JSON or Rust structs, which can be used for further analysis, storage, or integration into book-related applications.

## Technical description
The parser reads a markdown-like format with structured information for each book. Each book entry contains the following fields:

1. **Book Title**: The title of the book, enclosed in quotation marks.
2. **Authors**: A list of authors separated by commas, enclosed in square brackets.
3. **Genres**: A list of genres separated by commas, enclosed in square brackets.
4. **Publication Year**: The year the book was published as a positive integer.
5. **Rating**: The overall rating of the book as a floating-point value (0–10).
6. **Price**: The price of the book as a floating-point value, followed by the currency

### Grammar
The parser leverages the Pest library to handle the input format. The grammar rules defined in `grammar.pest` process various fields, including strings, numbers, and lists (e.g., authors and genres), ensuring accurate extraction.


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
  "price": "199.00 UAH"
}
```