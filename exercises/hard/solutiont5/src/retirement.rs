use regex::Regex;
pub fn retire_time(time: &str, tp: &str) -> String {
    // true: 男或法定退休年龄为55女职工, false: 原法定退休年龄为50女职工
    let mut case: bool = true;
    let parts: Vec<String> = time.split('-').map(|s| s.to_string()).collect();
    let year = parts[0].parse::<u32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();

    let mut end_year = 0;
    let mut end_month=0;
    let mut end_age: f32=0.0;
    let mut final_add_month=0;

    let re = Regex::new(r"原法定退休年龄(\d+)周岁女职工").unwrap();
    let mut work_age = 0;

    match re.captures(tp) {
        Some(caps) => {
            work_age = caps.get(1).map_or("0", |m| m.as_str()).parse().unwrap();
            if work_age != 55 {
                case = false;
            }
        }
        None => {
            // 男职工
            work_age = 60;
        }
    }

    // 计算退休年龄
    if year + work_age < 2025 {
        end_year = year + work_age;
        end_month = month;
        end_age = work_age as f32;
        final_add_month = 0;
    } else if year + work_age >= 2025 {
        let mut ave_month = 4;
        let mut max_year = 58;
        if !case {
            ave_month = 2;
            max_year = 55;
        } else if work_age == 60 {
            max_year = 63;
        }

        let over_month = (year + work_age- 2025) * 12 + month;
        let mut add_month = (over_month -1) / ave_month + 1;
        if add_month >= (max_year - work_age) * 12 {
            add_month = (max_year - work_age) *12;
        }
        let add_year = (add_month + month - 1) / 12;
        let add_month_t = (add_month + month - 1) %12 + 1;

        end_year = year + work_age + add_year;
        end_month = add_month_t;
        final_add_month = add_month;
        end_age = work_age as f32 + (add_month as f32) / (12 as f32);
    }

    let mut res;
    if end_age == end_age.trunc() {
        let end_age: u32 = end_age as u32;
        if end_month <10 {
            res = format!("{}-0{},{},{}", end_year, end_month, end_age, final_add_month);
        } else {
            res = format!("{}-{},{},{}", end_year, end_month, end_age, final_add_month);
        }

    } else {
        if end_month <10 {
            res = format!("{}-0{},{:.2},{}", end_year, end_month, end_age, final_add_month);
        } else {
            res = format!("{}-{},{:.2},{}", end_year, end_month, end_age, final_add_month);
        }
    }

    res
}
