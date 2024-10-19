use std::{collections::HashMap, io::stdin, vec::Vec};

fn main() {
    let (median,mode) = median_mode(&[1,2,3,4,5,6,6,6,7,7]).unwrap();
    println!("First function: median={median} mode={mode}");

    println!("Second function: apple={}, first={}", pig_latin("apple").unwrap(), pig_latin("first").unwrap());

    println!("Third function: ");
    start_interface();
}

fn median_mode(list: &[i32]) -> Option<(f32, i32)> {
    if list.is_empty() {
        return None;
    }

    let mut vector = Vec::new();
    let mut counts = HashMap::new();

    for elem in list {
        vector.push(*elem);
        let count = counts.entry(elem).or_insert(0);
        *count += 1;
    }

    vector.sort();
    let median: f32 = {
        let pos = vector.len() / 2;
        let half = *vector.get(pos).expect("value should exist") as f32;

        if vector.len() % 2 != 0 || pos == 0 {
            half
        } else {
            (half + *vector.get(pos-1).expect("value should exist") as f32) / 2.0
        }
    };

    let mut mode = vector.first().expect("Vec should be populated");

    for (k, v) in &counts {
        if v > counts.get(mode).expect("HashMap should be populated") {
            mode = k;
        }
    }
    Some((median, *mode))
}


fn starts_with_vowel(word: &str) -> bool {
    let vowels = ['a','e','i','o','u'];

    vowels.iter().any(|v| word.starts_with(*v))
}

fn pig_latin(word: &str) -> Option<String> {
    if word.is_empty() {
        return None;
    }

    if starts_with_vowel(word) {
        let mut s = String::from(word);
        s.push_str("-hay");
        Some(s)
    } else {
        let s = format!("{}-{}ay", &word[1..], &word[0..1]);
        Some(s)
    }
}


fn start_interface() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!();
        println!("Add, List, or Delete? ('exit' to quit): ");

        let input = get_input();

        match input.to_lowercase().trim() {
            "add" => add_employee(&mut departments),
            "list" => list_employees(&mut departments),
            "delete" => delete_employee(&mut departments),
            "exit" => {
                println!("Exiting from interface");
                break;
            },
            _ => println!("\nInvalid operation.")
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failure to read line");

    input.trim_end().to_string()
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>){
    let name = get_input();

    println!("to ");

    let department = get_input();

    let employees = departments.entry(department).or_default();

    employees.push(name);
}

fn print_departments(departments: &HashMap<String, Vec<String>>){
    for dept in departments.keys() {
        println!("{dept}");
    }
}

fn list_employees(departments: &mut HashMap<String, Vec<String>>){
    println!("List all people in a department (1) or all people in the company by department (2)?");

    let input = get_input();

    match input.trim().parse::<i32>() {
        Ok(1) => {
            println!("Which department?");
            print_departments(departments);
            println!();

            let department = get_input();

            let list = match departments.get_mut(&department) {
                Some(employees) => {
                    employees.sort();
                    employees
                },
                None => &mut Vec::new()
            };

            println!("{:#?}", &list);
        },
        Ok(2) => {
            for (dept, list) in departments.iter_mut() {
                list.sort();
                println!("{dept}");
                println!("{:#?}", &list);
            }
        },
        _ => {
            println!("Invalid value");
        }
    };
}

fn delete_employee(departments: &mut HashMap<String, Vec<String>>){
    println!("Which department?");
    print_departments(departments);
    println!();

    let dept = get_input();

    let employee_list: &mut Vec<String> = match departments.get_mut(&dept) {
        Some(employees) => {
            employees.sort();
            employees
        },
        None => {
            println!("{dept} not found.");
            &mut Vec::new()
        }
    };

    if employee_list.is_empty() {
        return;
    }
    
    println!("Which employee?");
    println!("{:#?}", &employee_list);

    let name = get_input();

    if !employee_list.is_empty() {
        match employee_list.binary_search(&name) {
            Ok(index) => {
                employee_list.remove(index);
                println!("Removed {} from {}", name, dept);
            },
            Err(_) => println!("Employee not found.")
        };
    }
}
