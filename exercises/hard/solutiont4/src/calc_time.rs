pub fn time_info(time: &str) -> String {
    let month_days: Vec<u32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let month_days_com: Vec<u32> = vec![0, 1, 0, 0, 0, 1, 1, 2, 3, 3, 4, 4];

    // 记录部分新年距1月1日期
    let new_years = vec![vec![2025, 29], vec![2026, 48]];
    // 部分假期
    let vacation = vec![vec![121, 5], vec![357, 10]];
    let mut res_str: Vec<String> = vec![];
    let parts: Vec<String> = time.split('-').map(|s| s.to_string()).collect();
    let year = parts[0].parse::<u32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();
    let bias = ((year - 1900) * 365 + (year - 1900 - 1) / 4) % 7; //
                                                                  //println!("bias: {}", bias);

    let this_year_is_leap: bool = year % 4 == 0;

    let mut pass_month_days: u32 = (month - 1) * 30 + month_days_com[(month - 1) as usize];
    if month > 2 && this_year_is_leap {
        pass_month_days += 1;
    } // 闰年自动填补空缺的一天
      //println!("pass month days: {}", pass_month_days);

    let mut this_month_bias: u32 = (pass_month_days + bias) % 7;
    let mut n_this_week = (pass_month_days + bias + day + 6) / 7; // 第几周
                                                                  //let n_this_week = (n_this_week - 1) % 5 + 1;
    let mut n_this_week_t: u32 = (this_month_bias + day + 6) % 7 + 1; // 周几
    let year_days_pass: u32 = pass_month_days + day; // 本年的第几天

    let mut this_year_days_left: u32 = 365 - year_days_pass; // 还剩多少天
    if this_year_is_leap {
        this_year_days_left += 1;
    }

    if this_year_days_left < 3 {
        n_this_week = 1;
    }

    let mut days_to_new_year: u32 = 0; // 距离过年
    for i in 0..new_years.len() {
        if year == new_years[i][0] {
            if year_days_pass <= new_years[i][1] {
                days_to_new_year = new_years[i][1] - year_days_pass;
            } else {
                days_to_new_year = new_years[i + 1][1] + this_year_days_left;
            }
        }
    }

    let mut stock_time = 0;
    if n_this_week_t == 6 || n_this_week_t == 7 || n_this_week_t == 5 {
        stock_time = 7 - n_this_week_t;
    }
    for i in 0..new_years.len() {
        if year == new_years[i][0]
            && year_days_pass >= new_years[i][1] - 1
            && year_days_pass <= new_years[i][1] + 7
        {
            stock_time = new_years[i][1] + 7 - year_days_pass - 1;
        }
    }
    for i in 0..vacation.len() {
        if year_days_pass >= vacation[i][0] - 1
            && year_days_pass <= vacation[i][0] + vacation[i][1] - 1
        {
            stock_time = vacation[i][0] + vacation[i][1] - year_days_pass - 1;
        }
    }

    res_str.push(n_this_week.to_string());
    res_str.push(n_this_week_t.to_string());
    res_str.push(year_days_pass.to_string());
    res_str.push(this_year_days_left.to_string());
    res_str.push(days_to_new_year.to_string());
    res_str.push(stock_time.to_string());

    res_str.join(",")
}
