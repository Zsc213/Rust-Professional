use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let l = input_str.len();
    let mut res: usize = 0;
    let mut c = HashSet::with_capacity(l);
    for i in input_str.chars() {
        if c.insert(i) {
            res += 1;
        }
    }
    res
}
