use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::BufReader;
use ypbank_parser::{BinaryFormat, CsvFormat, Format, TextFormat, Transaction};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    file1: String,

    #[arg(long)]
    format1: FormatType,

    #[arg(long)]
    file2: String,

    #[arg(long)]
    format2: FormatType,
}

#[derive(ValueEnum, Clone, Debug)]
enum FormatType {
    Binary,
    Csv,
    Text,
}

fn read_transactions(
    filename: &str,
    format_type: FormatType,
) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let transactions = match format_type {
        FormatType::Binary => BinaryFormat.read_from(reader)?,
        FormatType::Csv => CsvFormat.read_from(reader)?,
        FormatType::Text => TextFormat.read_from(reader)?,
    };
    Ok(transactions)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let transactions1 = read_transactions(&args.file1, args.format1)?;
    let transactions2 = read_transactions(&args.file2, args.format2)?;

    if transactions1 == transactions2 {
        println!("Транзакции {} и {} одинаковы.", args.file1, args.file2);
    } else {
        println!("Транзакции {} и {} разные.", args.file1, args.file2);

        if transactions1.len() != transactions2.len() {
            println!(
                "Количество записей: {} и {}.",
                transactions1.len(),
                transactions2.len()
            );
        } else {
            for (i, (t1, t2)) in transactions1.iter().zip(transactions2.iter()).enumerate() {
                if t1 != t2 {
                    println!("Различие в записи № {}:", i + 1);
                    println!("  File1: {:?}", t1);
                    println!("  File2: {:?}", t2);
                }
            }
        }

        std::process::exit(1);
    }
    Ok(())
}
