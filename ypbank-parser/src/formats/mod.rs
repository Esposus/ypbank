mod binary;
mod csv;
mod text;

use crate::{ParseResult, Transaction};
use std::io::{Read, Write};

/// Общий трейт для всех форматов парсинга
pub trait Format {
    /// Читает форматы их любого источника, реализующего Read
    fn read_from<R: Read>(&self, reader: R) -> ParseResult<Vec<Transaction>>;

    /// Записывает транзакции в любой приемник, реализующий Write
    fn write_to<W: Write>(&self, writer: W, transactions: &[Transaction]) -> ParseResult<()>;
}
