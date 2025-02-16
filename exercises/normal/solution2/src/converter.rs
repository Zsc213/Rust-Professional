use scanf::scanf;
pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    let inv = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let (num, base_a) = scanf!(num_str, "{}({})", &str, i32).unwrap();
    let Ok(num) = i32::from_str_radix(num, base_a);
    let Ok(res_str) = num.to_string_radix(to_base);
    res_str
}
