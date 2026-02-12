use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use ypbank_parser::{BinaryFormat, CsvFormat, Format, TextFormat};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input_file = File::open(&args.input)?;
    let reader = BufReader::new(input_file);
    let input_format = args.input_format.get_format();
    let transactions = input_format.read_from(reader)?;
    let output_format = args.output_format.get_format();
    let stdout = io::stdout();
    let writer = BufWriter::new(stdout);
    output_format.write_to(writer, &transactions)?;

    Ok(())
}
