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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input_file = File::open(&args.input)?;
    let reader = BufReader::new(input_file);

    let transactions = match args.input_format {
        FormatType::Binary => BinaryFormat.read_from(reader)?,
        FormatType::Csv => CsvFormat.read_from(reader)?,
        FormatType::Text => TextFormat.read_from(reader)?,
    };

    let stdout = io::stdout();
    let writer = BufWriter::new(stdout);
    match args.output_format {
        FormatType::Binary => BinaryFormat.write_to(writer, &transactions)?,
        FormatType::Csv => CsvFormat.write_to(writer, &transactions)?,
        FormatType::Text => TextFormat.write_to(writer, &transactions)?,
    }

    Ok(())
}
