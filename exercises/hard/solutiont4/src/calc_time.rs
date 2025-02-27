pub fn time_info(time: &str) -> String {
    let month_days: Vec<u32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let month_days_com: Vec<u32> = vec![0, 1, 0, 0, 0, 1, 1, 2, 3, 3, 4, 4];
    let mut res_str: Vec<String> = vec![];
    let parts: Vec<String> = time.split('-').map(|s| s.to_string()).collect();
    let year = parts[0].parse::<u32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();
    let bias = ((year - 1900) * 365 + (year - 1900 - 1) / 4) % 7; // 1月1日偏移

    let this_year_is_leap: bool = year % 4 == 0;

    let mut pass_month_days: u32 = bias + (month - 1) * 30 + month_days_com[(month - 1) as usize];
    if month > 2 && !this_year_is_leap {
        pass_month_days -= 1;
    } // 闰年自动填补空缺的一天
    let mut this_month_bias: u32 = pass_month_days % 7;
    let n_this_week = (this_month_bias + day) / 7 + 1; // 第几周
    let n_this_week_t: u32 = (this_month_bias + day + 6) % 7 + 1; // 周几
    let year_days_pass: u32 = pass_month_days - bias + day; // 本年的第几天

    let mut this_year_days_left: u32 = 365 - year_days_pass; // 还剩多少天
    if this_year_is_leap {
        this_year_days_left += 1;
    }
    let mut days_to_new_year: u32 = 0; // 距离过年
    if month < 2 {
        days_to_new_year = 31 - day + 4;
    } else if month == 2 && day <= 4 {
        days_to_new_year = day - 4;
    } else {
        days_to_new_year = this_year_days_left + 31 + 4;
    }

    let mut A_left = 0;
    if n_this_week_t == 6 || n_this_week_t == 7 {
        A_left = 7 - n_this_week_t;
    }

    res_str.push(n_this_week.to_string());
    res_str.push(n_this_week_t.to_string());
    res_str.push(year_days_pass.to_string());
    res_str.push(this_year_days_left.to_string());
    res_str.push(days_to_new_year.to_string());
    res_str.push(A_left.to_string());

    res_str.join(",")
}
