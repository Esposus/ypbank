use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::BufReader;
use ypbank_parser::Format;

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

impl From<FormatType> for Format {
    fn from(f: FormatType) -> Self {
        match f {
            FormatType::Binary => Format::Binary,
            FormatType::Csv => Format::Csv,
            FormatType::Text => Format::Text,
        }
    }
}

fn read_transactions(
    filename: &str,
    format_type: FormatType,
) -> Result<Vec<ypbank_parser::Transaction>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let format: Format = format_type.into();
    let transactions = format.read_from(reader)?;
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
                "Разное количество записей: {} и {} для первой и второй транзакции соответственно.",
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
