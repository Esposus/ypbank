use crate::{ParseError, ParseResult, Transaction, TransactionStatus, TransactionType};
use std::io::{BufRead, BufReader, Read, Write};

/// Парсер для CSV формата YPBankCsv
pub struct CsvFormat;

impl CsvFormat {
    /// Парсит строку CSV
    fn parse_csv_line(line: &str) -> ParseResult<Transaction> {
        let parts: Vec<&str> = line.split(",").collect();
        if parts.len() != 8 {
            return Err(ParseError::InvalidFormat(
                "Неверное количество полей в CSV".to_string(),
            ));
        }

        let tx_id = parts[0].parse()?;
        let tx_type = TransactionType::try_from(parts[1])?;
        let from_user_id = parts[2].parse()?;
        let to_user_id = parts[3].parse()?;
        let amount = parts[4].parse()?;
        let timestamp = parts[5].parse()?;
        let status = TransactionStatus::try_from(parts[6])?;
        let description = parts[7].trim_matches('"').to_string();

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

    /// Форматирует транзакцию в CSV строку
    fn format_transaction(transaction: &Transaction) -> String {
        format!(
            "{tx_id},{tx_type},{from_user_id},{to_user_id},{amount},{timestamp},{status},\"{description}\"",
            tx_id = transaction.tx_id,
            tx_type = transaction.tx_type,
            from_user_id = transaction.from_user_id,
            to_user_id = transaction.to_user_id,
            amount = transaction.amount,
            timestamp = transaction.timestamp,
            status = transaction.status,
            description = transaction.description,
        )
    }
}

impl super::Format for CsvFormat {
    fn read_from<R: Read>(&self, reader: R) -> ParseResult<Vec<Transaction>> {
        let mut transactions = Vec::new();
        let buf_reader = BufReader::new(reader);

        let mut lines = buf_reader.lines();
        if lines.next().is_none() {
            return Ok(transactions);
        }

        for line in lines {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let transaction = Self::parse_csv_line(line)?;
            transactions.push(transaction);
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(&self, mut writer: W, transactions: &[Transaction]) -> ParseResult<()> {
        write!(
            writer,
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
        )?;

        for transaction in transactions {
            writeln!(writer, "{}", Self::format_transaction(transaction))?;
        }

        Ok(())
    }
}
