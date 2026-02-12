mod binary;
mod csv;
mod text;

use crate::{ParseResult, Transaction};
use std::io::{Read, Write};

/// Доступные форматы финансовых данных
#[derive(Debug, Clone, Copy)]
pub enum Format {
    Binary,
    Csv,
    Text,
}

/// Общий трейт для всех форматов парсинга
impl Format {
    /// Читает форматы их любого источника, реализующего Read
    pub fn read_from<R: Read>(&self, reader: R) -> ParseResult<Vec<Transaction>> {
        match self {
            Format::Binary => binary::BinaryFormat.read_from(reader),
            Format::Csv => csv::CsvFormat.read_from(reader),
            Format::Text => text::TextFormat.read_from(reader),
        }
    }

    /// Записывает транзакции в любой приемник, реализующий Write
    pub fn write_to<W: Write>(&self, writer: W, transactions: &[Transaction]) -> ParseResult<()> {
        match self {
            Format::Binary => binary::BinaryFormat.write_to(writer, transactions),
            Format::Csv => csv::CsvFormat.write_to(writer, transactions),
            Format::Text => text::TextFormat.write_to(writer, transactions),
        }
    }
}

pub(crate) trait FormatParser {
    fn read_from<R: Read>(&self, reader: R) -> ParseResult<Vec<Transaction>>;
    fn write_to<W: Write>(&self, write: W, transactions: &[Transaction]) -> ParseResult<()>;
}
