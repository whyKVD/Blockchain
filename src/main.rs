use std::io;
use crate::{
    block::Block,
    transaction::Transaction,
    wallet::Wallet,
    blockchain::BlockChain
};

mod block;
mod wallet;
mod transaction;
mod blockchain;

fn read() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(error) => println!("Error: {}", error),
    }

    input.trim().to_string()
}

fn create_transaction() -> Transaction{
    println!("insert from_addr:\t");
    let from_addr = read();
    println!("insert to_addr:\t");
    let to_addr = read();
    println!("insert the amount:\t");
    let amount = read().as_str().parse::<i64>().expect("Not a number!");

    Transaction::new(from_addr, to_addr, amount)
}

fn mine_block(blockchain: &mut BlockChain, transactions: &mut Vec<Transaction>){
    if !blockchain.is_valid() {
        println!("Error blockchain is not valid!");
        return
    }

    blockchain.add_block(transactions.to_vec());

    println!("block mined succesfully");
}

fn menu() -> String {
    println!("transaction: 1, mine: 2, check balance: 3, new wallet: 4, blockchain status: 5, quit: q");

    read()
}

fn is_valid(bal: i64, transactions: &mut Vec<Transaction>, transaction: &mut Transaction) -> bool {
    let mut balance : i64 = bal;

    for t in transactions.to_vec().iter() {
        if transaction.from_addr.eq(&t.from_addr) { balance -= t.amount };
        if transaction.from_addr.eq(&t.to_addr) { balance += t.amount };
    }

    if transaction.amount > balance { return false };

    true
}

fn main() {
    let mut blockchain = BlockChain::new();
    let mut transactions : Vec<Transaction> = Vec::new();

    loop{
        match menu().as_str() {
            "1" => {
                let mut transaction : Transaction = create_transaction();
                if !is_valid(blockchain.get_balance(&transaction.from_addr), &mut transactions, &mut transaction) {
                    println!("Transazione rifiutata");
                    continue;
                }
                transactions.push(transaction);
            },
            "2" => {
                mine_block(&mut blockchain, &mut transactions);
                transactions = Vec::new();
            },
            "3" => {
                println!("wich balance you want to check:");

                println!("balance:\t{}", blockchain.get_balance(&read()));
            },
            "4" => {
                println!("new wallet created");
                println!("address: {:#?}", Wallet::new());
            },
            "5" => {
                println!("blockchain status:");
                println!("{:#?}", blockchain);
            }
            "q" => break,
            _ => println!("not in the menu"),
        }
    }

    println!("Blockchain is valid: {}", blockchain.is_valid());
    println!("{:#?}", blockchain);
}
