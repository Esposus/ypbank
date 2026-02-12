# YPBank Parser

Библиотека для парсинга, сериализации и десериализации финансовых данных с поддержкой трёх форматов. Проект разработан в рамках учебного задания для получения практических навыков работы со статическим полиморфизмом на основе трейтов в Rust.

## Возможности

### Поддержка трёх форматов финансовых данных:

- YPBankCsv — табличный формат CSV

- YPBankText — текстовый формат с парами ключ-значение
- YPBankBin — компактный бинарный форма

* Статический полиморфизм — все операции чтения/записи работают с любыми типами, реализующими Read/Write
* Идиоматичный Rust — обработка ошибок через Result, трейты, перечисления
* Гибкость — конвертация между любыми форматами
* Документированность — все публичные API покрыты документацией
* Тестирование — модульные тесты для каждого формата

### Использование конвертера

```bash
# Конвертация CSV в бинарный формат
cargo run --bin ypbank-converter -- \
  --input transactions.csv \
  --input-format csv \
  --output-format binary \
  > transactions.bin

# Конвертация бинарного формата в текстовый
cargo run --bin ypbank-converter -- \
  --input transactions.bin \
  --input-format binary \
  --output-format text \
  > transactions.txt

# Конвертация текстового формата в CSV
cargo run --bin ypbank-converter -- \
  --input transactions.txt \
  --input-format text \
  --output-format csv \
  > transactions.csv
```

### Сравнение файлов

```bash
# Сравнение бинарного файла с CSV
cargo run --bin ypbank-compare -- \
  --file1 transactions.bin \
  --format1 binary \
  --file2 transactions.csv \
  --format2 csv

# Успешное сравнение:
# Различное количество записей для 'transactions.bin' и 'transactions.csv'.

# При различиях:
# 'transactions.bin' and 'transactions.csv' различаются.
# Количество записей: 5 и 4
```
