# ypbank-parser

Библиотека для парсинга, сериализации и десериализации финансовых данных в трёх форматах: YPBankCsv, YPBankText, YPBankBin.

## Возможности

- Поддержка трёх форматов: CSV, текстовый (key-value), бинарный.
- Единый интерфейс через перечисление `Format`.
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
```rust
use ypbank_parser::{Format, Transaction};
use std::fs::File;
use std::io::BufReader;

// Чтение CSV-файла
let file = File::open("transactions.csv")?;
let reader = BufReader::new(file);
let transactions = Format::Csv.read_from(reader)?;

// Обработка
for tx in &transactions {
    println!("{}: {} {}", tx.tx_id, tx.tx_type, tx.amount);
}

// Запись в бинарный формат
let output = File::create("transactions.bin")?;
Format::Binary.write_to(output, &transactions)?;
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
