pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    const MONEY: [u32; 8] = [100, 50, 30, 20, 10, 5, 2, 1];
    let mut amount_value = amount;
    let mut account = 0;
    for i in 0..MONEY.len() {
        while amount_value > 0 && amount_value >= MONEY[i] {
            amount_value -= MONEY[i];
            account += 1;
        }
    }
    account
}
