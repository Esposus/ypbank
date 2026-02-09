use thiserror::Error;

// Тип результата для операций парсинга
pub type ParseResult<T> = std::result::Result<T, ParseError>;

// Ошибка парсинга финансовых данных
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO ошибка: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 ошибка: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Ошибка парсинга числа: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Неверный формат: {0}")]
    InvalidFormat(String),

    #[error("Неверный тип транзакции: {0}")]
    InvalidTransactionType(String),

    #[error("Неверный статус транзакции: {0}")]
    InvalidTransactionStatus(String),

    #[error("Магическое число не совпадает")]
    InvalidMagic,

    #[error("Размер записи не совпадает")]
    RecordSizeMismatch,

    #[error("Поле не найдено: {0}")]
    MissingField(String),
}
