use crate::error::ParseError;
use std::fmt;

/// Финансовая транзакция
#[derive(Debug, PartialEq, Clone)]
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

/// Тип финансовой транзакции
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransactionType {
    Deposit,
    Transfer,
    Withdrawal,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "DEPOSIT"),
            TransactionType::Transfer => write!(f, "TRANSFER"),
            TransactionType::Withdrawal => write!(f, "WITHDRAWAL"),
        }
    }
}

impl TryFrom<&str> for TransactionType {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "TRANSFER" => Ok(TransactionType::Transfer),
            "WITHDRAWAL" => Ok(TransactionType::Withdrawal),
            _ => Err(ParseError::InvalidTransactionType(value.to_string())),
        }
    }
}

impl TryFrom<u8> for TransactionType {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TransactionType::Deposit),
            1 => Ok(TransactionType::Transfer),
            2 => Ok(TransactionType::Withdrawal),
            _ => Err(ParseError::InvalidTransactionType(value.to_string())),
        }
    }
}

impl From<TransactionType> for u8 {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Deposit => 0,
            TransactionType::Transfer => 1,
            TransactionType::Withdrawal => 2,
        }
    }
}

/// Статус транзакции
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransactionStatus {
    Success,
    Failure,
    Pending,
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionStatus::Success => write!(f, "SUCCESS"),
            TransactionStatus::Failure => write!(f, "FAILURE"),
            TransactionStatus::Pending => write!(f, "PENDING"),
        }
    }
}

impl TryFrom<&str> for TransactionStatus {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SUCCESS" => Ok(TransactionStatus::Success),
            "FAILURE" => Ok(TransactionStatus::Failure),
            "PENDING" => Ok(TransactionStatus::Pending),
            _ => Err(ParseError::InvalidTransactionStatus(value.to_string())),
        }
    }
}

impl TryFrom<u8> for TransactionStatus {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TransactionStatus::Success),
            1 => Ok(TransactionStatus::Failure),
            2 => Ok(TransactionStatus::Pending),
            _ => Err(ParseError::InvalidTransactionStatus(value.to_string())),
        }
    }
}

impl From<TransactionStatus> for u8 {
    fn from(value: TransactionStatus) -> u8 {
        match value {
            TransactionStatus::Success => 0,
            TransactionStatus::Failure => 1,
            TransactionStatus::Pending => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_type_from_str() {
        assert_eq!(TransactionType::try_from("DEPOSIT").unwrap(), TransactionType::Deposit);
        assert_eq!(TransactionType::try_from("TRANSFER").unwrap(), TransactionType::Transfer);
        assert_eq!(TransactionType::try_from("WITHDRAWAL").unwrap(), TransactionType::Withdrawal);
        assert!(TransactionType::try_from("INVALID").is_err());
    }

    #[test]
    fn test_transaction_type_from_u8() {
        assert_eq!(TransactionType::try_from(0u8).unwrap(), TransactionType::Deposit);
        assert_eq!(TransactionType::try_from(1u8).unwrap(), TransactionType::Transfer);
        assert_eq!(TransactionType::try_from(2u8).unwrap(), TransactionType::Withdrawal);
        assert!(TransactionType::try_from(3u8).is_err());
    }

    #[test]
    fn test_transaction_type_into_u8() {
        assert_eq!(Into::<u8>::into(TransactionType::Deposit), 0u8);
        assert_eq!(Into::<u8>::into(TransactionType::Transfer), 1u8);
        assert_eq!(Into::<u8>::into(TransactionType::Withdrawal), 2u8);
    }

    // Аналогично для TransactionStatus
}
