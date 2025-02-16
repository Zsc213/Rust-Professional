use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let l = input_str.len();
    let mut res: usize = 0;
    let mut c = HashSet::<&str>::with_capacity(l);
    let str_s = input_str.split(',');
    for i in str_s {
        if c.insert(i) {
            res += 1;
        }
    }
    res
}
