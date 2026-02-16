# ypbank-parser

Библиотека для парсинга, сериализации и десериализации финансовых данных в трёх форматах: YPBankCsv, YPBankText, YPBankBin.

## Возможности

- Поддержка трёх форматов: CSV, текстовый (key-value), бинарный.
- Единый интерфейс через трейт`Format`.
- Все операции чтения/записи работают с любыми типами, реализующими `std::io::Read` и `std::io::Write`.
- Полная обработка ошибок без `unwrap()`.
- Модульные тесты для каждого формата.

## Установка

Добавьте зависимость в `Cargo.toml`:

```toml
[dependencies]
ypbank-parser = { path = "../ypbank-parser" }
```

## Использование

// Чтение CSV-файла

```rust
let file = File::open("transactions.csv")?;
let reader = BufReader::new(file);
let transactions = CsvFormat.read_from(reader)?;

// Обработка транзакций
for transaction in &transactions {
println!("{}: {} {}", transaction.tx_id, transaction.tx_type, transaction.amount);
}

// Запись в бинарный формат
let output = File::create("transactions.bin")?;
BinaryFormat.write_to(output, &transactions)?;

// Также можно работать с другими форматами
let text_file = File::open("transactions.txt")?;
let text_reader = BufReader::new(text_file);
let text_transactions = TextFormat.read_from(text_reader)?;

let bin_file = File::open("transactions.bin")?;
let bin_reader = BufReader::new(bin_file);
let bin_transactions = BinaryFormat.read_from(bin_reader)?;
```

## Структуры данных

###Transaction

Основная структура, представляющая финансовую транзакцию:

```rust
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub description: String,
}
```

## Перечисления

`TransactionType`: `Deposit`, `Transfer`, `Withdrawal`

`TransactionStatus`: `Success`, `Failure`, `Pending`

`Format`: `Binary`, `Csv`, `Text` — для выбора формата

## Тестирование

Запустите тесты:

```bash
cargo test
```

## Лицензия

The MIT License (MIT)
