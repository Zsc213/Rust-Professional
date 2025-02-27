//I AM NOT DONE
//Calculated according to ISO8061 standard

use crate::calc_time::time_info;

mod calc_time;

fn main() {
    let st = time_info("2025-01-18");
    println!("{}", st);
}
