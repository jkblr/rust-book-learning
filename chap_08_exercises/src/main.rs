use std::collections::HashMap;

fn pig_latin_converter(s: &str) -> String {
    const CONSONANTS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut still_in_prefix = true;
    let mut prefix = String::new();
    let mut suffix = String::new();
    for c in s.chars() {
        if still_in_prefix && CONSONANTS.contains(&c) {
            suffix.push(c);
            still_in_prefix = false;
        } else if still_in_prefix {
            prefix.push(c);
        } else {
            suffix.push(c);
        }
    }
    let appendage = if prefix.is_empty() { "hay" } else { "ay" };
    format!("{suffix}{prefix}-{appendage}")
}

fn median_and_mode(mut v: Vec<i32>) -> (i32, i32) {
    let len = v.len();
    v.sort();
    let median = if len % 2 == 0 {
        (v[len / 2 - 1] + v[len / 2]) / 2
    } else {
        v[len / 2]
    };
    let mut map = HashMap::new();
    for val in v.iter().copied() {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }
    let mut max_counter = 0;
    let mut mode = v[0];
    for (key, value) in &map {
        if *value > max_counter {
            mode = *key;
            max_counter = *value;
        }
    }

    (median, mode)
}

fn main() {
    //1st Exercise:
    //let v = vec![1, 11, 12, 3, 4, 10, 3, 3, 6, 34, 6, 15, 7686, 1, 2];
    //let res = median_and_mode(v);
    //println!("The median is {} and the mode is {}", res.0, res.1);
    //
    //2nd Exercise:
    let word1: String = String::from("chair");
    let word2: String = String::from("apple");
    let res1 = pig_latin_converter(&word1);
    let res2 = pig_latin_converter(&word2);
    println!("The results are {} and {}", res1, res2);
}
