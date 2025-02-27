use indexmap::IndexMap;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const NUM: usize = 5;

type CityGroups = HashMap<String, HashMap<String, Vec<String>>>;
type CityGroups_t = Vec<(String, Vec<(String, Vec<String>)>)>;

fn merge_link(link_record: &Vec<Vec<usize>>, n: usize) -> usize {
    let mut count: usize = 0;
    let mut m: Vec<usize> = vec![0; n];
    for i in 0..link_record.len() {
        if m[link_record[i][0]] == 0 {
            // 初始组
            count += 1;
            m[link_record[i][0]] = count.clone();
            m[link_record[i][1]] = count.clone();
        } else {
            m[link_record[i][1]] = m[link_record[i][0]].clone();
        }
    }
    count
}

pub fn count_provinces() -> String {
    let json_content: CityGroups = {
        let file_json = fs::read_to_string("./district2.json").expect("read error");
        serde_json::from_str::<CityGroups>(&file_json).expect("serialize error")
    };
    let mut res_str: Vec<String> = vec![String::new(); NUM];

    for (group_id, cities) in &json_content {
        // 分组
        let mut res: usize = 0;
        let mut count: usize = 0;
        let mut link_record: Vec<Vec<usize>> = vec![];
        let mut st: HashMap<String, usize> = HashMap::new();
        for (city, links) in cities {
            if st.contains_key(city) {
                link_record.push(vec![st.get(city).unwrap().clone(), count.clone()]);
                st.insert(city.to_string(), count.clone());
            } else {
                st.insert(city.to_string(), count.clone());
            }
            for city_name in links.iter() {
                if st.contains_key(city_name) {
                    link_record.push(vec![st.get(city_name).unwrap().clone(), count.clone()]);
                    st.insert(city_name.to_string(), count.clone());
                } else {
                    st.insert(city_name.to_string(), count.clone());
                }
            }
            count += 1;
        }

        // 分析link_record
        let id: usize = group_id.parse().unwrap();
        res = merge_link(&link_record, count);
        res_str[id - 1] = res.to_string();

        // if res_str != String::from("") {
        //     res_str += &(",".to_owned() + &res.to_string());
        // } else {
        //     res_str += &res.to_string();
        // }
    }

    res_str.join(",")
}
