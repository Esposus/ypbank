use crate::{Transaction, TransactionStatus, TransactionType, ParseResult};

use std::io::{Read, Write};

pub struct BinaryFormat;

impl BinaryFormat {
    fn read_transaction<R: Read>(reader: &mut R) -> ParseResult<Transaction> {

        Ok(Transaction{
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

    fn write_transaction<W: Write>(&self, writer: &mut writer, transaction: &Transaction) -> ParseResult<()> {
        Ok(())
    }
}

impl super::Format for BinaryFormat {
    fn read_from<R:Read>(&self, mut reader: R) -> ParseResult<Vec<Transaction>> {
        Ok()
    }

    fn write_to<W: Write>(&self, mut writer: W, transactions: &[Transaction]) -> ParseResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParseResult;
    use std::io::Cursor;
    use crate::{Transaction, TransactionType, TransactionStatus};

    #[test]
    fn test_binary_roundtrip() -> ParseResult<()> {
        let transaction = Transaction {
            tx_id: 123456,
            tx_type: TransactionType::Deposit,
            from_user_id: 0,
            to_user_id: 789,
            amount: 10_000,
            timestamp: 1633036800000,
            status: TransactionStatus::Success,
            description: "Test transaction".to_string(),
        }

        let format = BinaryFormat;
        let mut buffer = Vec::new();

        format.write_to(&mut buffer, &[transaction.clone()])?;

        let result = format.read_from(Cursor::new(buffer))?;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], transaction);

        Ok(())
    }
}
