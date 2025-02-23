pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut res: u32 = 0;
    let mut m = amount.clone();
    let money = vec![100, 50, 30, 20, 10, 5, 2, 1];
    for i in 0..money.len() {
        res += m / money[i];
        m %= money[i];
    }
    res
}
