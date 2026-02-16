mod binary;
mod csv;
mod text;

use crate::{ParseResult, Transaction};
use std::io::{Read, Write};

pub use binary::BinaryFormat;
pub use csv::CsvFormat;
pub use text::TextFormat;

/// Общий трейт для всех форматов парсинга
pub trait Format {
    /// Читает транзакции из любого источника, реализующего Read
    fn read_from<R: Read>(&self, reader: R) -> ParseResult<Vec<Transaction>>;
    /// Записывает транзакции в любой приемник, реализующий Write
    fn write_to<W: Write>(&self, write: W, transactions: &[Transaction]) -> ParseResult<()>;
}
