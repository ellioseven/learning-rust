struct Mastercard {
    id: u8
}

struct BitPay {
    btc_number: u32
}

impl CreditCharge for BitPay {

    fn charge_with_id(&self, id: u32) -> bool {
        return id % 2 == self.btc_number % 2
    }

}

trait CreditCharge {

    fn charge_with_id(&self, id: u32) -> bool;

}

fn transact<T: CreditCharge>(card: T) {
    let code = 4096;
    if card.charge_with_id(code) {
        println!("Success.");
    } else {
        println!("Failure.");
    }

}

fn main () {
    let card = BitPay { btc_number: 1024 };
    transact(card);
}
