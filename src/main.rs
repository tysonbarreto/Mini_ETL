use axum_etl::model;
use axum_etl::etl;

use anyhow::Result;


fn main()->Result<()>{

    let transactions = etl::read_transactions_from_csv("artifacts/data.csv")?;
    println!("Loaded {} transactions", transactions.len());

    for row in transactions.into_iter(){
        print!("{:#?}",row);
    }
    Ok(())
}