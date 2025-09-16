struct Account {
    balance : f64,
}

impl Account {
    fn deposit(&mut self, amount : f64){
        self.balance = amount;
        println!("Deposito {} a su cuenta",amount);
    }

    fn withdraw(&mut self, amount : f64) -> Result<(), String>{
        if amount > self.balance {
            Err("Saldo Insuficiente".to_string())
        } else { 
            self.balance -= amount;
            Ok(())
        }
    }

    fn my_balance(&self){
        println!("Su saldo es {}", self.balance);
    }
}

fn main() {
    let mut account = Account { balance : 0.0 };
    let amount = 500.0;
    
    account.my_balance();

    account.deposit(1000.0);
    
    account.my_balance();

    match account.withdraw(amount) {
        Ok(()) => println!("Se retiro {} de su cuenta", amount),
        Err(err) => println!("{}", err),
    }
    
    account.my_balance();
}
