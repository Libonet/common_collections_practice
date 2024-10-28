use std::{collections::HashMap, io::stdin, vec::Vec};

pub fn median_mode(list: &[i32]) -> Option<(f32, i32)> {
    if list.is_empty() {
        return None;
    }

    let mut counts = HashMap::new();
    let mut vector = Vec::new();
    
    for elem in list {
        vector.push(elem);

        let count = counts.entry(elem).or_insert(0);
        *count += 1;
    }

    vector.sort();

    let median: f32 = {
        let pos = vector.len() / 2;
        let half = **vector.get(pos).expect("value should exist") as f32;

        if vector.len() % 2 != 0 || pos == 0 {
            half
        } else {
            (half + **vector.get(pos-1).expect("value should exist") as f32) / 2.0
        }
    };

    let (mode, _) = counts.iter().max_by(|(_,v1),(_,v2)| v1.cmp(v2)).expect("HashMap should be populated");

    Some((median, **mode))
}

#[cfg(test)]
mod median_mode_tests {
    use super::median_mode;

    #[test]
    fn median_of_odd_list() {
        let list = [1,2,3,4,5];

        assert_eq!(3.0, median_mode(&list).unwrap().0);
    }

    #[test]
    fn median_of_even_list() {
        let list = [1,2,3,4,5,6];

        assert_eq!(3.5, median_mode(&list).unwrap().0);
    }

    #[test]
    fn empty_list() {
        assert_eq!(None, median_mode(&[]));
    }
}

fn starts_with_vowel(word: &str) -> bool {
    let vowels = ['a','e','i','o','u'];

    vowels.iter().any(|v| word.starts_with(*v))
}

pub fn pig_latin(word: &str) -> Option<String> {
    if word.is_empty() {
        return None;
    }

    if starts_with_vowel(word) {
        let mut s = String::from(word);
        s.push_str("-hay");
        Some(s)
    } else {
        let mut letters = word.chars();
        let first_letter = letters.next().expect("Word should not be empty");

        let rest = &word[first_letter.len_utf8()..];

        let s = format!("{}-{}ay", rest, first_letter);
        Some(s)
    }
}

#[cfg(test)]
mod pig_latin_tests {
    use super::pig_latin;

    #[test]
    fn vowel() {
        assert_eq!("apple-hay", pig_latin("apple").unwrap());
    }

    #[test]
    fn consonant() {
        assert_eq!("uck-fay", pig_latin("fuck").unwrap());
    }

    #[test]
    fn empty_str() {
        assert_eq!(None, pig_latin(""));
    }
}


pub fn start_interface() {
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
    employees.sort();
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
                    employees
                },
                None => &mut Vec::new()
            };

            println!("{:#?}", &list);
        },
        Ok(2) => {
            for (dept, list) in departments.iter_mut() {
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

