use crate::{ParseError, ParseResult, Transaction, TransactionStatus, TransactionType};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

///Парсер для бинарного формата YPBankBin
pub struct BinaryFormat;

impl BinaryFormat {
    /// Читает одну транзакцию из бинарного формата
    fn read_transaction<R: Read>(reader: &mut R) -> ParseResult<Transaction> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        if magic != MAGIC {
            return Err(ParseError::InvalidMagic);
        }

        let _record_size = reader.read_u32::<BigEndian>()?;

        let tx_id = reader.read_u64::<BigEndian>()?;
        let tx_type_byte = reader.read_u8()?;
        let tx_type = TransactionType::try_from(tx_type_byte)?;
        let from_user_id = reader.read_u64::<BigEndian>()?;
        let to_user_id = reader.read_u64::<BigEndian>()?;
        let amount = reader.read_i64::<BigEndian>()?;
        let timestamp = reader.read_u64::<BigEndian>()?;
        let status_byte = reader.read_u8()?;
        let status = TransactionStatus::try_from(status_byte)?;
        let description_len = reader.read_u32::<BigEndian>()?;

        let mut description_bytes = vec![0u8; description_len as usize];
        reader.read_exact(&mut description_bytes)?;
        let description = String::from_utf8(description_bytes)?;

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

    /// Записывает одну транзакцию в бинарном формате
    fn write_transaction<W: Write>(
        &self,
        writer: &mut W,
        transaction: &Transaction,
    ) -> ParseResult<()> {
        writer.write_all(&MAGIC)?;

        let description_bytes = transaction.description.as_bytes();
        let record_size = 8 + 1 + 8 + 8 + 8 + 8 + 1 + 4 + description_bytes.len();

        writer.write_u32::<BigEndian>(record_size as u32)?;

        writer.write_u64::<BigEndian>(transaction.tx_id)?;
        writer.write_u8(transaction.tx_type.into())?;
        writer.write_u64::<BigEndian>(transaction.from_user_id)?;
        writer.write_u64::<BigEndian>(transaction.to_user_id)?;
        writer.write_i64::<BigEndian>(transaction.amount)?;
        writer.write_u64::<BigEndian>(transaction.timestamp)?;
        writer.write_u8(transaction.status.into())?;
        writer.write_u32::<BigEndian>(description_bytes.len() as u32)?;
        writer.write_all(description_bytes)?;

        Ok(())
    }
}

impl super::Format for BinaryFormat {
    fn read_from<R: Read>(&self, mut reader: R) -> ParseResult<Vec<Transaction>> {
        let mut transactions = Vec::new();

        while let Ok(transaction) = Self::read_transaction(&mut reader) {
            transactions.push(transaction);
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(&self, mut writer: W, transactions: &[Transaction]) -> ParseResult<()> {
        for transaction in transactions {
            self.write_transaction(&mut writer, transaction)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParseResult;
    use crate::formats::Format;
    use crate::{Transaction, TransactionStatus, TransactionType};
    use std::io::Cursor;

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
        };

        let format = BinaryFormat;
        let mut buffer = Vec::new();

        format.write_to(&mut buffer, &[transaction.clone()])?;

        let result = format.read_from(Cursor::new(buffer))?;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], transaction);

        Ok(())
    }
}
