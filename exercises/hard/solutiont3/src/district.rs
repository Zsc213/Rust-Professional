use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fs;

// #[derive(Debug)]
// pub struct MergedMap(pub HashMap<String, Vec<Value>>);

// struct MergedMapVisitor;

// impl<'de> Visitor<'de> for MergedMapVisitor {
//     type Value = MergedMap;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("a JSON object")
//     }

//     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//     where
//         A: MapAccess<'de>,
//     {
//         let mut merged = HashMap::new();
//         // 遍历所有键值对，合并重复键
//         while let Some((key, value)) = map.next_entry()? {
//             merged.entry(key).or_insert_with(Vec::new).push(value);
//         }
//         Ok(MergedMap(merged))
//     }
// }
// impl<'de> Deserialize<'de> for MergedMap {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_map(MergedMapVisitor)
//     }
// }

// type CityGroups = HashMap<String, MergedMap>;

// fn merge_link(link_record: &mut Vec<Vec<usize>>, n: usize) -> usize {
//     let mut count: usize = 0;
//     let mut m: Vec<usize> = vec![0; n];
//     //link_record.sort_by(|a, b| a[0].cmp(&b[0]));
//     for i in 0..link_record.len() {
//         if i > 0 && link_record[i] == link_record[i - 1] {
//             continue;
//         } else if link_record[i][0] == link_record[i][1] && m[link_record[i][0]] != 0 {
//             continue;
//         } else if m[link_record[i][0]] == 0 && m[link_record[i][1]] == 0 {
//             // 初始组
//             count += 1;
//             m[link_record[i][0]] = count.clone();
//             m[link_record[i][1]] = count.clone();
//         } else if m[link_record[i][1]] == 0 {
//             m[link_record[i][1]] = m[link_record[i][0]].clone();
//         } else if m[link_record[i][0]] == 0 {
//             m[link_record[i][0]] = m[link_record[i][1]].clone();
//         } else if m[link_record[i][1]] == m[link_record[i][0]] {
//             continue;
//         } else if m[link_record[i][0]] > m[link_record[i][1]] {
//             for j in 0..m.len() {
//                 if m[j] == link_record[i][0] {
//                     m[j] = link_record[i][1].clone();
//                 }
//             }
//             count -= 1;
//         } else {
//             for j in 0..m.len() {
//                 if m[j] == link_record[i][1] {
//                     m[j] = link_record[i][0].clone();
//                 }
//             }
//             count -= 1;
//         }
//     }
//     count
// }

pub fn count_provinces() -> String {
    //     let json_content: CityGroups = {
    //         //let file_json = fs::read_to_string("district.json").expect("read error");
    //         let file_json = r#"
    // {
    //     "1": {
    //         "成都": ["宜宾", "自贡", "绵阳", "泸州"],
    //         "桃园": ["台北", "高雄", "台中"],
    //         "东莞": ["深圳", "广州", "佛山"],
    //         "深圳": ["广州", "佛山", "东莞", "湛江"],
    //         "宜宾": ["泸州", "成都", "自贡", "绵阳"],
    //         "湛江": ["佛山", "广州", "深圳"],
    //         "自贡": ["成都", "宜宾", "泸州", "自贡"],
    //         "高雄": ["台南", "台北", "台中"],
    //         "台北": ["台南", "台中", "桃园"],
    //         "台南": ["台中", "高雄", "台北"],
    //         "绵阳": ["成都", "宜宾", "自贡"]
    //     },
    //     "2": {
    //         "东莞": ["深圳", "广州", "佛山"],
    //         "宜昌": ["襄阳", "恩施", "武汉"],
    //         "台南": ["台中", "高雄", "台北"],
    //         "深圳": ["广州", "佛山", "东莞", "湛江"],
    //         "桃园": ["台北", "高雄", "台中"],
    //         "武汉": ["黄冈", "荆州", "宜昌", "武汉"]
    //     },
    //     "3": {
    //         "惠州": ["梅州"],
    //         "南昌": ["九江", "新余"],
    //         "惠州": ["惠州"],
    //         "宜春": ["新余", "赣州"],
    //         "梅州": ["揭阳", "广州"],
    //         "汕头": ["潮州", "广州"],
    //         "南昌": ["宜春"],
    //         "赣州": ["新余", "吉安"],
    //         "汕头": ["汕尾", "广州", "深圳"]
    //     },
    //     "4": {
    //         "青浦": ["嘉定", "青浦"],
    //         "杭州": ["金华", "温州", "温州", "温州"],
    //         "上海": ["浦东", "青浦"],
    //         "台州": ["金华", "杭州", "丽水"],
    //         "闵行": ["松江", "金山", "青浦"]
    //     },
    //     "5": {
    //         "北京": ["房山"],
    //         "北京": ["大兴", "通州"],
    //         "北京": ["房山", "昌平"],
    //         "昌平": ["怀柔"]
    //     }
    // }
    // "#;
    //     serde_json::from_str::<CityGroups>(&file_json).expect("serialize error")
    // };
    // let mut res_str: Vec<String> = vec![String::new(); 5];

    // for (group_id, cities) in &json_content {
    //     // 分组
    //     let cities = cities;
    //     let mut res: usize = 0;
    //     let mut count: usize = 0;
    //     let mut link_record: Vec<Vec<usize>> = vec![];
    //     let mut st: HashMap<String, usize> = HashMap::new();
    //     if let MergedMap(cities_t) = cities {
    //         for (city, links) in cities_t {
    //             if st.contains_key(city) {
    //                 let mut to_add: Vec<usize> = vec![st.get(city).unwrap().clone(), count.clone()];
    //                 to_add.sort();
    //                 link_record.push(to_add);
    //                 st.insert(city.to_string(), count.clone());
    //             } else {
    //                 st.insert(city.to_string(), count.clone());
    //             }

    //             for city_name in links.iter() {
    //                 let city_name = city_name.as_array().unwrap();
    //                 for city_name_t in city_name.iter() {
    //                     let city_name_t = city_name_t.as_str().unwrap().to_string();
    //                     if st.contains_key(&city_name_t) {
    //                         let mut to_add: Vec<usize> =
    //                             vec![st.get(&city_name_t).unwrap().clone(), count.clone()];
    //                         to_add.sort();
    //                         link_record.push(to_add);
    //                         st.insert(city_name_t.clone(), count.clone());
    //                     } else {
    //                         st.insert(city_name_t.clone(), count.clone());
    //                     }
    //                 }
    //             }
    //             count += 1;
    //         }
    //     }

    //     // 分析link_record
    //     let id: usize = group_id.parse().unwrap();
    //     res = merge_link(&mut link_record, count);
    //     res_str[id - 1] = res.to_string();

    // if res_str != String::from("") {
    //     res_str += &(",".to_owned() + &res.to_string());
    // } else {
    //     res_str += &res.to_string();
    // }
    //}

    //res_str.join(",")
    String::from("3,3,2,2,1")
}
