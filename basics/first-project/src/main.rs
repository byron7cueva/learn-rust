// Enumaration
/* enum Payment {
  Cash,
  CreditCard,
  DebitCard,
}

fn main () {
  let some_payment = Payment::Cash;

  match some_payment {
    Payment::Cash => {
      println!("Paying with cash...");
    }
    Payment::CreditCard => {
      println!("Paying with credit card...");
    }
    /* Payment::DebitCard => {
      println!("Paying with debit card...");
    }*/
    _ => {}
  }
}*/

enum Payment {
  Cash(f32), // Value
  CreditCard(String, f32), // Tuple
  DebitCard(DebitData), // Struct
  Crypto{account_id: String, amount: f32},
}

struct DebitData {
  pub card_number: String,
  pub amount: f32
}

fn main () {
  let some_payment = Payment::Cash(100.);
  proccess_payment(some_payment);

  let cc_payment = Payment::CreditCard("CC Num".to_string(), 250.);
  proccess_payment(cc_payment);

  let debit_payment = Payment::DebitCard(DebitData {
    card_number: "Debit num".to_string(),
    amount: 100.,
  });
  proccess_payment(debit_payment);

  let crypto_payment = Payment::Crypto{account_id: "abc 123".to_string(), amount: 20.};
  proccess_payment(crypto_payment);
}

fn proccess_payment(some_payment: Payment) {
  match some_payment {
    Payment::Cash(amt) => {
      println!("Paying with cash... in the amount of {}", amt);
    }
    /* Payment::CreditCard(dsc, amt) => {
      println!("Paying with credit card... Desc is {} and amount is {} ", dsc, amt);
    } */
    /* Payment::CreditCard(dsc, _amt) => { //Poniendole _amt no valida si se esta utilizando el parametro
      println!("Paying with credit card... Desc is {}", dsc);
    } */
    Payment::CreditCard(dsc, _) => { //Poniendole _ no valida que le hace falta otro parametro de usar
      println!("Paying with credit card... Desc is {}", dsc);
    }
    Payment::DebitCard(data) => {
      println!("Paying with debit card... card_number {}, amount {}", data.card_number, data.amount);
    }
    Payment::Crypto{account_id, amount} => {
      println!("Paying with crypto... account_id {}, amount {}", account_id, amount);
    }
  }
}