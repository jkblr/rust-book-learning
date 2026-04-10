use std::collections::HashMap;
use std::io;

// pig latin exercise
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

///////////////////////////////////////////////////////////////////////////

// median and mode exercise
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

////////////////////////////////////////////////////////////////////////////

//company names exercise

//Helper function for adding Name to vector and sorting
fn extend_and_sort_vector(v: &mut Vec<String>, s: String) {
    v.push(s);
    v.sort();
}

// Helper function for integrating name, department pair into existing hashmap
fn add_person_to_department(
    person: String,
    department: String,
    map: &mut HashMap<String, Vec<String>>,
) {
    let current_department_vector = map.entry(department).or_insert(Vec::new());
    extend_and_sort_vector(current_department_vector, person);
}

fn validate_string_format(s: &str) -> Vec<String> {
    let mut counter = 0;
    let mut res = Vec::new();
    for word in s.split_whitespace() {
        if counter == 0 || counter == 2 {
            res.push(String::from(word));
        }
        counter += 1;
    }
    res
}

fn io() {
    let mut map = HashMap::new();
    let mut get_list = true;
    while get_list {
        println!("Please input a sentence of the form 'Add Sally to Engineering':");

        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Failed to read line!");
        let trimmed_input = a.trim();
        if &trimmed_input == &"List" {
            get_list = false;
        //TODO: Add Printing here
        } else {
            let inputs = validate_string_format(trimmed_input);
            add_person_to_department(inputs[0].clone(), inputs[1].clone(), &mut map);
        }
    }
}

fn main() {
    //1st Exercise:
    //let v = vec![1, 11, 12, 3, 4, 10, 3, 3, 6, 34, 6, 15, 7686, 1, 2];
    //let res = median_and_mode(v);
    //println!("The median is {} and the mode is {}", res.0, res.1);
    //
    //2nd Exercise:
    //let word1: String = String::from("chair");
    //let word2: String = String::from("apple");
    //let res1 = pig_latin_converter(&word1);
    //let res2 = pig_latin_converter(&word2);
    //println!("The results are {} and {}", res1, res2);
    //
    //3rd Exercise:
    io();
}
