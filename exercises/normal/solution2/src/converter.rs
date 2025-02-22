use sscanf::scanf;

fn vert_to_ten(num_str: &str, n: i32) -> i32 {
    let st: String = String::from(num_str);
    let mut res: i32 = 0;
    if n == 10 {
        res = st.parse::<i32>().unwrap();
    } else if n < 10 {
        let mut temp: u32 = 0;
        for i in st.chars().rev() {
            res += n.pow(temp) * i.to_digit(10).unwrap() as i32;
            temp += 1;
        }
    }
    res
}
fn vert_to_n(num: i32, n: u32) -> String {
    let mut res = String::new();
    let mut num_t = num.clone();

    loop {
        let remainder = (num_t % n as i32) as u32;
        let digit = match remainder {
            0..=9 => char::from_digit(remainder, 10).unwrap(),
            _ => char::from_u32(remainder - 10 + 'a' as u32).unwrap(),
        };
        res.push(digit);
        num_t /= n as i32;
        if num_t == 0 {
            break;
        }
    }
    res.chars().rev().collect()
}

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑

    let (num, base_a) = scanf!(num_str, "{}({})", &str, i32).unwrap();
    let res = vert_to_ten(num, base_a);
    vert_to_n(res, to_base)
}
