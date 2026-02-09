mod binary;
mod csv;
mod text;

use crate::{ParseResult, Transaction};
use std::io::{Read, Write};

pub trait Format {
    fn read_from<R: Read>(&self, reader: R) -> ParseResult<Vec<Transaction>>;

    fn write_to<W: Write>(&self, writer: W, transactions: &[Transaction]) -> ParseResult<()>;
}
