use std::{path::Path};
use anyhow::{Ok, Result};
use csv::{self,ReaderBuilder};
use std::fs::File;

use crate::model::{RawTransaction,Transaction};

pub fn read_transactions_from_csv<P:AsRef<Path>>(path:P)->Result<Vec<Transaction>>{
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut transactions = Vec::new();

    for result in reader.deserialize::<RawTransaction>(){
        let raw_transaction_row = result?;
        let transaction_row = Transaction::from(raw_transaction_row);
        transactions.push(transaction_row)
    }
    Ok(transactions)
}