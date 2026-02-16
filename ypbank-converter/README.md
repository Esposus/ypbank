# ypbank-converter

Консольная утилита для конвертации файлов с финансовыми данными между поддерживаемыми форматами.

## Установка

Соберите проект из исходников:

```bash
git clone git@github.com:Esposus/ypbank.git
cd ypbank
cargo build --release
```

Бинарный файл будет находиться в `target/release/ypbank-converter`.

## Использование

```bash
ypbank-converter --input <INPUT_FILE> --input-format <FORMAT> --output-format <FORMAT>
```

## Аргументы

`-i`, `--input` Путь к входному файлу
`--input-format` Формат входного файла: `binary`, `csv`, `text`
`--output-format` Желаемый формат выходных данных: `binary`, `csv`, `text`
Результат выводится в stdout. Для сохранения в файл используйте перенаправление `>`.

## Примеры

### Конвертация CSV в бинарный формат

```bash
ypbank-converter \
  --input transactions.csv \
  --input-format csv \
  --output-format binary \
  > transactions.bin
```

### Конвертация бинарного формата в текстовый

```bash
ypbank-converter \
  --input transactions.bin \
  --input-format binary \
  --output-format text \
  > transactions.txt
```

### Конвертация текстового формата в CSV

```bash
ypbank-converter \
  --input transactions.txt \
  --input-format text \
  --output-format csv \
  > transactions.csv
```

## Обработка ошибок

При возникновении ошибки (например, неверный формат файла, повреждённые данные) программа выведет сообщение об ошибке в stderr и завершится с ненулевым кодом.

## Лицензия

The MIT License (MIT)
