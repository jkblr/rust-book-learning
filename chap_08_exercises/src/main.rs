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

enum Inputs {
    ListDepartment(&'static String),
    ListAll,
    InputNameDepartment(&'static String, &'static String),
    InvalidInput,
}

fn validate_string_format(s: &str) -> Inputs {
    let mut res_vec = Vec::new();
    for word in s.split_whitespace() {
        res_vec.push(String::from(word));
    }
    if res_vec.len() == 4 && { &res_vec[0] == "Add" && &res_vec[3] == "to" } {
        return Inputs::InputNameDepartment(&res_vec[1], &res_vec[3]);
    } else if res_vec.len() == 3 && { &res_vec[0] == "List" && &res_vec[1] == "Department" } {
        return Inputs::ListDepartment(&res_vec[2]);
    } else if res_vec.len() == 2 && { &res_vec[0] == "List" && &res_vec[1] == "All" } {
        return Inputs::ListAll;
    } else {
        return Inputs::InvalidInput;
    }
}

fn io() {
    let mut map = HashMap::new();
    loop {
        println!("Please input a sentence of the form 'Add Sally to Engineering' to input data, or list all employees with 'List All' or list a specific department's employees with 'List Department <Department Name>'");

        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Failed to read line!");
        let trimmed_input = a.trim();
        let case = validate_string_format(trimmed_input);
        match case {
            Inputs::ListAll => {
                for (key, value) in &map {
                    println!("These are the employees in department {}", key);
                    for name in value {
                        println!("{}", name);
                    }
                }

                //TODO: Add Printing here
            }
            Inputs::InputNameDepartment(name, department) => {
                add_person_to_department(*name, *department, &mut map);
            }
            Inputs::ListDepartment(department) => {
                for (key, value) in &map {
                    if key == &department {
                        println!("These are the employees in department {}", key);
                        for name in value {
                            println!("{}", name);
                        }
                    }
                }
            }
            Inputs::InvalidInput => continue,
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
