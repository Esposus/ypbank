use crate::{ParseError, ParseResult, Transaction, TransactionStatus, TransactionType};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

/// Парсер для текстового формата YPBankText
pub struct TextFormat;

impl TextFormat {
    /// Парсит блок текста в транзакцию
    fn parse_text_block(block: &str) -> ParseResult<Transaction> {
        let mut fields = HashMap::new();

        for line in block.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, ':').map(|s| s.trim()).collect();

            if parts.len() != 2 {
                continue;
            }

            fields.insert(parts[0], parts[1]);
        }

        let get_field = |key: &str| {
            fields
                .get(key)
                .ok_or_else(|| ParseError::MissingField(key.to_string()))
        };

        let tx_id = get_field("TX_ID")?.parse()?;
        let tx_type = TransactionType::try_from(*get_field("TX_TYPE")?)?;
        let from_user_id = get_field("FROM_USER_ID")?.parse()?;
        let to_user_id = get_field("TO_USER_ID")?.parse()?;
        let amount = get_field("AMOUNT")?.parse()?;
        let timestamp = get_field("TIMESTAMP")?.parse()?;
        let status = TransactionStatus::try_from(*get_field("STATUS")?)?;
        let description = get_field("DESCRIPTION")?.trim_matches('"').to_string();

        Ok(Transaction {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        })
    }
}

impl super::FormatParser for TextFormat {
    fn read_from<R: Read>(&self, reader: R) -> ParseResult<Vec<Transaction>> {
        let mut transactions = Vec::new();
        let buf_reader = BufReader::new(reader);
        let mut current_block = String::new();

        for line in buf_reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                if !current_block.is_empty() {
                    let transaction = Self::parse_text_block(&current_block)?;
                    transactions.push(transaction);
                    current_block.clear();
                }
            } else {
                current_block.push_str(&line);
                current_block.push('\n');
            }
        }

        if !current_block.is_empty() {
            let transaction = Self::parse_text_block(&current_block)?;
            transactions.push(transaction);
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(&self, mut writer: W, transactions: &[Transaction]) -> ParseResult<()> {
        for (i, transaction) in transactions.iter().enumerate() {
            if i > 0 {
                writeln!(writer)?;
            }

            writeln!(writer, "TX_ID: {}", transaction.tx_id)?;
            writeln!(writer, "TX_TYPE: {}", transaction.tx_type)?;
            writeln!(writer, "FROM_USER_ID: {}", transaction.from_user_id)?;
            writeln!(writer, "TO_USER_ID: {}", transaction.to_user_id)?;
            writeln!(writer, "AMOUNT: {}", transaction.amount)?;
            writeln!(writer, "TIMESTAMP: {}", transaction.timestamp)?;
            writeln!(writer, "STATUS: {}", transaction.status)?;
            writeln!(writer, "DESCRIPTION: {}", transaction.description)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParseResult;
    use crate::formats::FormatParser;
    use ::std::io::Cursor;

    #[test]
    fn test_text_roundtrip() -> ParseResult<()> {
        let transaction = vec![Transaction {
            tx_id: 1234567890123456,
            tx_type: TransactionType::Deposit,
            from_user_id: 0,
            to_user_id: 9876543210987654,
            amount: 10_000,
            timestamp: 1633036800000,
            status: TransactionStatus::Success,
            description: "Terminal deposit".to_string(),
        }];
        let format = TextFormat;
        let mut buffer = Vec::new();

        format.write_to(&mut buffer, &transaction)?;

        let result = format.read_from(Cursor::new(buffer))?;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], transaction[0]);

        Ok(())
    }
    #[test]
    fn test_text_multiple_records_with_comments() {
        let data = "\
# Record 1
TX_ID: 1
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 100
AMOUNT: 1000
TIMESTAMP: 1000
STATUS: SUCCESS
DESCRIPTION: \"first\"

# Record 2
TX_ID: 2
TX_TYPE: WITHDRAWAL
FROM_USER_ID: 100
TO_USER_ID: 0
AMOUNT: 200
TIMESTAMP: 2000
STATUS: PENDING
DESCRIPTION: \"\"
";
        let format = TextFormat;
        let txs = format.read_from(Cursor::new(data)).unwrap();
        assert_eq!(txs.len(), 2);
        assert_eq!(txs[0].tx_id, 1);
        assert_eq!(txs[1].description, "");
    }

    #[test]
    fn test_text_missing_field() {
        let data = "\
TX_ID: 1
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 100
AMOUNT: 1000
TIMESTAMP: 1000
STATUS: SUCCESS
";
        let format = TextFormat;
        let result = format.read_from(Cursor::new(data));
        assert!(matches!(result, Err(ParseError::MissingField(field)) if field == "DESCRIPTION"));
    }

    #[test]
    fn test_text_invalid_field_order() {
        let data = "\
TX_TYPE: DEPOSIT
TX_ID: 1
FROM_USER_ID: 0
TO_USER_ID: 100
AMOUNT: 1000
TIMESTAMP: 1000
STATUS: SUCCESS
DESCRIPTION: \"test\"
";
        let format = TextFormat;
        let txs = format.read_from(Cursor::new(data)).unwrap();
        assert_eq!(txs.len(), 1);
    }

    #[test]
    fn test_text_invalid_enum() {
        let data = "\
TX_ID: 1
TX_TYPE: INVALID
FROM_USER_ID: 0
TO_USER_ID: 100
AMOUNT: 1000
TIMESTAMP: 1000
STATUS: SUCCESS
DESCRIPTION: \"test\"
";
        let format = TextFormat;
        let result = format.read_from(Cursor::new(data));
        assert!(matches!(result, Err(ParseError::InvalidTransactionType(_))));
    }
}
