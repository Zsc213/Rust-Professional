use primes;
pub fn goldbach_conjecture() -> String {
    let mut res = vec![];
    let mut count = 2;
    let mut i = 3;
    let mut primes_vec = vec![];
    loop {
        // 判断i是否为质数
        if primes::is_prime(i) {
            primes_vec.push(i);
        } else {
            let mut flag = false;
            for j in 0..primes_vec.len() {
                let t = (((i - primes_vec[j]) / 2) as f64).sqrt();
                if t.trunc() == t {
                    flag = true;
                }
            }
            if !flag {
                res.push(i.clone());
                count -= 1;
                if count == 0 {
                    break;
                }
            }
        }
        i += 2;
    }
    format!("{},{}", res[0], res[1])
}
