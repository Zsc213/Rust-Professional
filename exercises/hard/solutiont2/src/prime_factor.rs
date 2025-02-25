fn fm(mut num: u128, mut k: u128, p: u128) -> u128 {
    let mut res = 0;
    num %= p;
    while k != 0 {
        if k & 1 != 0 {
            res = (res + num) % p;
        }
        num = (num + num) % p;
        k /= 2;
    }
    res
}
fn fe(mut num: u128, mut exp: u128, p: u128) -> u128 {
    let mut res = 1;
    while exp != 0 {
        if exp & 1 != 0 {
            res = fm(res.clone(), num.clone(), p.clone());
        }
        exp /= 2;
        num = fm(num.clone(), num.clone(), p.clone());
    }
    res
}

fn fm_test(num: u128) -> bool {
    let test_vec: Vec<u128> = vec![3, 5, 7, 11];
    if num == 2 {
        return true;
    } else if num % 2 as u128 == 0 {
        return false;
    } else {
        for i in 0..test_vec.len() {
            if fe(test_vec[i].clone(), num - 1, num.clone()) != 1 {
                return false;
            }
            let mut y = num - 1;
            let mut temp: u128;
            //二次探测 temp 为num或num-1
            while y & 1 != 0 {
                y >>= 1;
                temp = fe(test_vec[i].clone(), y.clone(), num.clone());
                if temp != 1 && temp != num - 1 {
                    return false;
                } else if temp == num - 1 {
                    return true;
                }
            }
        }
    }
    true
}

pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut number_cut = number.clone();
    let mut res: u128 = number.clone();

    while number_cut % 2 == 0 {
        number_cut /= 2;
        res = 2;
    }
    let mut i: u128 = 3;
    while i <= number_cut && i <= 1000000 {
        while number_cut % i == 0 {
            res = i;
            number_cut /= i;
        }

        if number_cut == 1 {
            break;
        }
        if fm_test(number_cut.clone()) {
            res = number_cut;
            break;
        }
        i += 1;
    }
    res
    // let tes: u128 = 370373;
    // if fm_test(tes) {
    //     1
    // } else {
    //     0
    // }
}
