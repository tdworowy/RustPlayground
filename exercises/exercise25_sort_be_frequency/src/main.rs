use std::collections::HashMap;

fn frequency_sort(s: String) -> String {
    let mut f_map: HashMap<char, usize> = HashMap::new();
    s.chars().for_each(|ch| {
        *f_map.entry(ch).or_insert(0) += 1;
    });
   
    let mut count_vec: Vec<(&char, &usize)> = f_map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
   
    let mut r: String = "".to_string();
   
    count_vec.iter().for_each(|t| {
        for _ in 0..*t.1 {
            r += &t.0.to_string();
        }
    });
    r
}

fn main() {
    println!("{:?}", frequency_sort("tree".to_string()));
    println!("{:?}", frequency_sort("cccaaa".to_string()));
    println!("{:?}", frequency_sort("Aabb".to_string()));
}
