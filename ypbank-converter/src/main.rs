use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use ypbank_parser::{BinaryFormat, CsvFormat, TextFormat, Format};

#[derive(Parser, Debug)]
#[command(author, version, about = "Конвертер финансовых данных между форматами")]
struct Args {
    /// Входной файл
    #[arg(short, long)]
    input: String,

    /// Формат входного файла
    #[arg(long = "input-format")]
    input_format: FormatType,

    /// Формат выходного файла
    #[arg(long = "output-format")]
    output_format: FormatType,
}

#[derive(ValueEnum, Clone, Debug)]
enum FormatType {
    Binary,
    Csv,
    Text,
}

impl FormatType {
    fn get_format(&self) -> Box<dyn Format> {
        match self {
            FormatType::Binary => Box::new(BinaryFormat),
            FormatType::Csv => Box::new(CsvFormat),
            FormatType::Text => Box::new(TextFormat),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
