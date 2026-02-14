# ypbank-compare

Утилита для сравнения двух файлов с финансовыми данными, возможно, в разных форматах.

## Установка

Соберите проект из исходников:

```bash
git clone git@github.com:Esposus/ypbank.git
cd ypbank
cargo build --release
```
Бинарный файл будет находиться в `target/release/ypbank-compare`.

## Использование

```bash
ypbank-compare --file1 <FILE1> --format1 <FORMAT> --file2 <FILE2> --format2 <FORMAT>
```
## Аргументы

`--file1`	Путь к первому файлу
`--format1`	Формат первого файла: `binary`, `csv`, `text`
`--file2`	Путь ко второму файлу
`--format2`	Формат второго файла: `binary`, `csv`, `text`

## Примеры

### Сравнение бинарного файла с CSV

```bash
ypbank-compare \
  --file1 transactions.bin \
  --format1 binary \
  --file2 transactions.csv \
  --format2 csv
  ```
  ### Вывод при успехе
  
  ```text
  Транзакции 'transactions.bin' и 'transactions.csv' одинаковы.
  ```
  
  ### Вывод при различиях
  
  ```text
  Транзакции 'transactions.bin' и 'transactions.csv' разные.
  ```
  Программа завершается с кодом 0, если файлы идентичны, и с кодом 1, если различаются или произошла ошибка.
  
  ## Обработка ошибок
  При ошибках чтения файла или парсинга программа выведет сообщение в stderr и завершится с кодом 1.
