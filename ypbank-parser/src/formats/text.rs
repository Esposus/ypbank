use crate::{ParseResult, Transaction, TransactionType, TransactionStatus};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

/// Парсер для текстового формата YPBankText
pub struct TextFormat;

impl TexFormat {
    /// Парсит блок текста в транзакцию
    fn parse_text_block(block: &str) -> ParseResult<Transaction> {}
}

impl Format for TextFormat {
    fn read_from<R: Read>(&self, reader: R) -> ParseResult<Vec<Transaction>> {}

    fn write_to<W: Write>(&self, writer: W, transactions: &[Transaction]) -> ParseResult<()> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use::std::io::Cursor;
    use crate::ParseResult;

    #[test]
    fn test_text_roundtrip() -> ParseResult<()> {
        let transaction = vec![
            Transaction{
                tx_id: 1234567890123456,
                tx_type: TransactionType::Deposit,
                from_user_id: 0,
                to_user_id: 9876543210987654,
                amount: 10_000,
                timestamp: 1633036800000,
                status: TransactionStatus::Success,
                description: "Terminal deposit".to_string(),
            },
        ];
        let format = TextFormat;
        let mut buffer = Vec::new();

        format.write_to(&mut buffer, &transaction)?;

        let result = format.read_from(Cursor::new(buffer))?;



        assert_eq!(result.len(), 1);
        assert_eq!(result[0], transaction);

        Ok(())
    }
}