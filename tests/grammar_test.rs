use anyhow::anyhow;
use books_description_parser::*;
use pest::Parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() -> anyhow::Result<()> {
        // Тест на одиничний пробіл
        let pair = Grammar::parse(Rule::WHITESPACE, " ")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), " ");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 1);

        // Тест на табуляцію
        let pair = Grammar::parse(Rule::WHITESPACE, "\t")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "\t");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 1);

        // Тест на некоректний символ (наприклад, літера)
        let pair = Grammar::parse(Rule::WHITESPACE, "a");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::WHITESPACE, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_space() -> anyhow::Result<()> {
        // Тест на один або більше пробілів
        let pair = Grammar::parse(Rule::SPACE, "  ")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "  ");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::SPACE, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_book_title() -> anyhow::Result<()> {
        // Тест коректної назви книги
        let pair = Grammar::parse(Rule::book_title, "Book 1: \"Enemy Of My Enemy\"\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Book 1: \"Enemy Of My Enemy\"\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 28);

        let pair = Grammar::parse(Rule::book_title, "Book: \"Enemy Of My Enemy\"\n");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::book_title, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_publication_year() -> anyhow::Result<()> {
        // Тест коректного року публікації
        let pair = Grammar::parse(Rule::publication_year, "Publication Year: 2016\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Publication Year: 2016\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 23);

        // Тест некоректного формату року (без чисел)
        let pair = Grammar::parse(Rule::publication_year, "Publication Year:");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::publication_year, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_price() -> anyhow::Result<()> {
        // Тест на коректну ціну
        let pair = Grammar::parse(Rule::price, "Price: 199.00 UAH\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Price: 199.00 UAH\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 18);

        // Тест на цілу ціну
        let pair = Grammar::parse(Rule::price, "Price: 19 UAH\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Price: 19 UAH\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 14);

        // Тест на некоректну ціну (без "UAH")
        let pair = Grammar::parse(Rule::price, "Price: 299.99\n");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на некоректну ціну (без числа)
        let pair = Grammar::parse(Rule::price, "Price: UAH\n");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::price, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на ціну без Price
        let pair = Grammar::parse(Rule::price, "199.00 UAH\n");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_rating() -> anyhow::Result<()> {
        // Тест коректного рейтингу
        let pair = Grammar::parse(Rule::rating, "Rating: 9.5\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Rating: 9.5\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 12);

        // Тест некоректного рейтингу (значення вище 10)
        let pair = Grammar::parse(Rule::rating, "Rating: 10.5\n");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест некоректного формату (без значення)
        let pair = Grammar::parse(Rule::rating, "Rating:\n");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::rating, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_currency() -> anyhow::Result<()> {
        // Тест на валідний вхід
        let pair = Grammar::parse(Rule::currency, "USD")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "USD");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 3);

        // Тест на валідний вхід із довшою валютою
        let pair = Grammar::parse(Rule::currency, "UAH")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "UAH");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 3);

        // Тест на некоректну валюту
        let pair = Grammar::parse(Rule::currency, "123");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::currency, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_book_num() -> anyhow::Result<()> {
        // Тест на коректний номер книги
        let pair = Grammar::parse(Rule::book_num, "123")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "123");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 3);

        // Тест на довгий номер книги
        let pair = Grammar::parse(Rule::book_num, "9876543210")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "9876543210");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 10);

        // Тест на некоректний вхід (нецифрові символи)
        let pair = Grammar::parse(Rule::book_num, "ad");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::book_num, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_year() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::year, "2024")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "2024");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 4);

        // Тест на некоректний вхід (нецифрові символи)
        let pair = Grammar::parse(Rule::year, "ad");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::year, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_number() -> anyhow::Result<()> {
        // Тест на коректне число
        let pair = Grammar::parse(Rule::number, "199.00")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "199.00");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 6);

        // Тест на ціле число
        let pair = Grammar::parse(Rule::number, "100")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "100");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 3);

        // Тест на некоректне число
        let pair = Grammar::parse(Rule::number, "XXII");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на некоректне число
        let pair = Grammar::parse(Rule::number, ".4");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на від'ємне число
        let pair = Grammar::parse(Rule::number, "-100")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "-100");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 4);

        // Тест на від'ємне число з десятковими знаками
        let pair = Grammar::parse(Rule::number, "-199.99")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "-199.99");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 7);

        // Тест на порожній рядок
        let pair = Grammar::parse(Rule::number, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_quoted_text() -> anyhow::Result<()> {
        // Тест на текст у лапках
        let pair = Grammar::parse(Rule::quoted_text, "\"Hello, World!\"")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "\"Hello, World!\"");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 15);

        // Тест на порожній текст у лапках
        let pair = Grammar::parse(Rule::quoted_text, "\"\"")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "\"\"");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);

        // Тест на текст із символами
        let pair = Grammar::parse(Rule::quoted_text, "\"Some text!\"")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "\"Some text!\"");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 12);

        // Тест некоректного формату (відсутні лапки)
        let pair = Grammar::parse(Rule::quoted_text, "Hello, World!");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_rating_value() -> anyhow::Result<()> {
        // Тест на коректне значення рейтингу (ціле число)
        let pair = Grammar::parse(Rule::rating_value, "8")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "8");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 1);

        // Тест на коректне значення рейтингу (десяткове число)
        let pair = Grammar::parse(Rule::rating_value, "8.5")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "8.5");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 3);

        // Тест на некоректність рядка (буква)
        let pair = Grammar::parse(Rule::rating_value, "a");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на некоректне значення рейтингу (0, менше за мінімальне)
        let pair = Grammar::parse(Rule::rating_value, "-1");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_list_of_authors() -> anyhow::Result<()> {
        // Тест на коректний список авторів
        let pair = Grammar::parse(
            Rule::list_of_authors,
            "Authors: [John Smith, Alice Smith]\n",
        )?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Authors: [John Smith, Alice Smith]\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 35);

        // Тест на список з одним автором
        let pair = Grammar::parse(Rule::list_of_authors, "Authors: [John Smith]\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Authors: [John Smith]\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 22);

        // Тест на порожній список авторів
        let pair = Grammar::parse(Rule::list_of_authors, "Authors: []\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Authors: []\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 12);

        // Тест некоректного формату (відсутні дужки)
        let pair = Grammar::parse(Rule::list_of_authors, "Authors: John Smith, Alice Smith\n");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_author() -> anyhow::Result<()> {
        // Тест на коректне ім'я автора
        let pair = Grammar::parse(Rule::author, "John Smith")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "John Smith");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 10);

        // Тест на ім'я автора з символами
        let pair = Grammar::parse(Rule::author, "Dr. Alice")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Dr. Alice");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 9);

        // Тест на некоректний вхід (зайві дужки)
        let pair = Grammar::parse(Rule::author, "[John Smith]");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожнє ім'я автора
        let pair = Grammar::parse(Rule::author, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_list_of_genres() -> anyhow::Result<()> {
        // Тест на коректний список жанрів
        let pair = Grammar::parse(Rule::list_of_genres, "Genres: [Fantasy, Science Fiction]\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Genres: [Fantasy, Science Fiction]\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 35);

        // Тест на список із одним жанром
        let pair = Grammar::parse(Rule::list_of_genres, "Genres: [Fantasy]\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Genres: [Fantasy]\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 18);

        // Тест на порожній список жанрів
        let pair = Grammar::parse(Rule::list_of_genres, "Genres: []\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Genres: []\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 11);

        // Тест некоректного формату (відсутні дужки)
        let pair = Grammar::parse(Rule::list_of_genres, "Genres: Fantasy, Science Fiction\n");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_genre_item() -> anyhow::Result<()> {
        // Тест на коректний жанр
        let pair = Grammar::parse(Rule::genre_item, "Fantasy")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Fantasy");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 7);

        // Тест на жанр з пробілами
        let pair = Grammar::parse(Rule::genre_item, "Science Fiction")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), "Science Fiction");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 15);

        // Тест на некоректний вхід (зайві дужки)
        let pair = Grammar::parse(Rule::genre_item, "[Fantasy]");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній жанр
        let pair = Grammar::parse(Rule::genre_item, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_book() -> anyhow::Result<()> {
        let input = r#"Book 1: "Test Book Title"
Authors: [Author1, Author2]
Genres: [Fiction, Adventure]
Publication Year: 2023
Rating: 9.5
Price: 150.00 UAH
"#;

        let pair = Grammar::parse(Rule::book, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), input);
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), input.len());

        // Тест на неповний вхід
        let pair = Grammar::parse(Rule::book, "Book 1: \"Test Book Title\"");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній вхід
        let pair = Grammar::parse(Rule::book, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }

    #[test]
    fn test_books() -> anyhow::Result<()> {
        let input = r#"Book 1: "First Book"
Authors: [Author1, Author2]
Genres: [Fiction]
Publication Year: 2020
Rating: 8.0
Price: 120.00 UAH

Book 2: "Second Book"
Authors: [Author3]
Genres: [Non-Fiction, Biography]
Publication Year: 2021
Rating: 9.0
Price: 200.00 UAH

Book 3: "Third Book"
Authors: [Author3]
Genres: [Non-Fiction, Biography]
Publication Year: 2020
Rating: 8.5
Price: 220.00 UAH
"#;

        let pair = Grammar::parse(Rule::books, input)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        assert_eq!(pair.as_str(), input);
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), input.len());

        // Тест на неповний вхід
        let pair = Grammar::parse(
            Rule::books,
            r#"Book 1: "First Book"
Authors: [Author1, Author2]
Genres: [Fiction]
Publication Year: 2020
Rating: 8.0
Price: 120.00 UAH"#,
        );
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        // Тест на порожній вхід
        let pair = Grammar::parse(Rule::books, "");
        assert!(pair.is_err(), "Expected error but got {:?}", pair);

        Ok(())
    }
}
