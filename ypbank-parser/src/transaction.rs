use std::fmt;

// Тип финансовой транзакции
#[derive(Debug)]
pub enum TransactionType {
    Deposit,
    Transfer,
    Withdraw,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "DEPOSIT"),
            TransactionType::Transfer => write!(f, "TRANSFER"),
            TransactionType::Withdraw => write!(f, "WITHDRAW"),
        }
    }
}

// Статус транзакции
#[derive(Debug)]
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

// Финансовая транзакция
#[derive(Debug)]
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
