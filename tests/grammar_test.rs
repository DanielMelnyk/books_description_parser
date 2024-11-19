use pest::Parser;
use anyhow::anyhow;
use books_description_parser::*;


#[cfg(test)]
mod tests {
    use super::*;
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

}