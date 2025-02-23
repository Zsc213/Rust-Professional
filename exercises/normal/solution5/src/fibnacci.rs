pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut w: Vec<u32> = vec![0, 1];
    let mut res: u32 = 1;
    let mut temp: u32;
    loop {
        temp = w[0] + w[1];
        if temp > threshold {
            break;
        } else if temp % 2 == 1 {
            res += temp;
        }
        w[0] = w[1];
        w[1] = temp;
    }
    res
}
