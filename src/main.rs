use std::{collections::HashMap};
use std::io;
use std::io::Write;
use core::str::Chars;

fn main()
{
    /*
    Using a hash map and vectors, create a text interface to allow a user to add employee
    names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    Then let the user retrieve a list of all people in a department or all people in the company by
    department, sorted alphabetically.
    */

    let mut company_hashmap:HashMap<String, Vec<String>> = HashMap::new();
    let mut sales_department:Vec<String> = Vec::new();
    let mut human_resources_department:Vec<String> = Vec::new();
    let mut software_engineering_department:Vec<String> = Vec::new();
    let mut public_relations_department:Vec<String> = Vec::new();

    company_hashmap.insert(String::from("Sales"), sales_department);
    company_hashmap.insert(String::from("Human Resources"), human_resources_department);
    company_hashmap.insert(String::from("Software Engineering"), software_engineering_department);
    company_hashmap.insert(String::from("Public Relations"), public_relations_department);


    let mut employees:Vec<String> = vec![String::from("Elaine"), String::from("Antonio"),
        String::from("Wahlid"), String::from("Bridgitte"), String::from("Jennifer"),
        String::from("ben Maimon"), String::from("al-Mustafa"), String::from("Wolfgang"),
        String::from("Yvonne"), String::from("von Wright")];
        
    print_vector_str(&employees);

    println!("============================ SEPARATOR ============================");

    for (key, value) in &company_hashmap
    {
        println!("{}, {:#?}", key, value);
    }

    println!("============================ SEPARATOR ============================");

    ask_user_add_employees(&mut company_hashmap, &mut employees);

    println!("============================ SEPARATOR ============================");

    for (key, value) in company_hashmap
    {
        println!("{key}: {:#?}", value);
    }

    println!("============================ SEPARATOR ============================");

    println!("=>The employees vector is still available because the function ask_user_add_employees made \
    a copy of it and modified the copy instead of the original vector.");
    let mut i:usize = 0;
    while i < employees.len()
    {
        println!("{}", employees[i]);
        i+=1;
    }

}


fn ask_user_add_employees(company_hashmap_param:&mut HashMap<String, Vec<String>>, employees_param:&mut Vec<String>)
{
    let mut employees_param_copy:Vec<String> = copy_strs_vectors(employees_param);
    sort_strs_in_two_vectors(&mut employees_param_copy, employees_param);

    drop(employees_param_copy); // Deallocate the data inside this pointer to increase performance and free memory on the heap.
    // Indeed we can't use employees_param_copy from now on, which is exactly what we want.

    let mut employees_param_copy_2:Vec<String> = copy_strs_vectors(employees_param);
    let mut employees_param_copy_3:Vec<String> = employees_param.clone(); // We will use this down there!

    let company_hashmap_copy:HashMap<String, Vec<String>> = company_hashmap_param.clone();
    let mut vector_departments:Vec<String> = Vec::new();
    let mut key_to_copy:String = String::new();

    for (key, value) in company_hashmap_copy
    {
        key_to_copy = key.clone();
        vector_departments.push(key_to_copy);
    }

    let mut vector_departments_copy:Vec<String> = copy_strs_vectors(&mut vector_departments);
    sort_strs_in_two_vectors(&mut vector_departments_copy, &mut vector_departments);

    let mut user_doesnt_want_quit:bool = true;
    while user_doesnt_want_quit
    {
        if employees_param_copy_3.len() < 1
        {
            println!("===>There are no more employees to add. This function will now terminate.");
            break;
        }

        println!("Let's add employees to the company.");
        println!("Here is a list of all departments, sorted alphabetically:");
        let mut i:usize = 0;
        while i < vector_departments.len()
        {
            println!("  {}) {}", i + 1, vector_departments[i]);
            i+=1;
        }
    
        let mut choice:String = String::from("");
        let mut choice_to_integer:i32 = 0;
        warning_message();
    
        let smallest_possible_value:usize = 1;
        let greatest_possible_value:usize = vector_departments.len();
    
        while choice_to_integer < smallest_possible_value as i32 || choice_to_integer > greatest_possible_value as i32
        {
            choice = String::from("");
            choice_to_integer = 0;
            print!("  Which department would you like to add employees to? Please type in an integer number from 1 to {}: ", i);
            io::stdout().flush().unwrap(); // To be able to use print!() That is, without a line feed
            //https://stackoverflow.com/questions/37531903/how-do-i-print-output-without-a-trailing-newline-in-rust#:~:text=You%20can%20use%20the%20print,)%3B%20io%3A%3Astdin().
            io::stdin().read_line(&mut choice).expect("Failed to read line");
            print!("\n");
    
            choice_to_integer = choice.trim().parse().unwrap(); // Big number to make the crashing of the program more difficult
    
            if choice_to_integer < smallest_possible_value as i32 || choice_to_integer > greatest_possible_value as i32
            {
                println!("  => The company has {greatest_possible_value} departments. Please type in an integer \
                number from {smallest_possible_value} to {greatest_possible_value}. Here is the list of departments:");
                let mut i:usize = 0;
                while i < vector_departments.len()
                {
                    println!("  {}) {}", i + 1, vector_departments[i]);
                    i+=1;
                }
                print!("\n");
            }
        }
        
        let mut user_wants_add_employee_same_department:bool = true;
        while user_wants_add_employee_same_department
        {
            if employees_param_copy_3.len() < 1
            {
                println!("===>There are no more employees to add. This function will now terminate.");
                break;
            }

            println!("  Here is the list of employees:");
            let mut j:usize = 0;
            while j < employees_param_copy_3.len()
            {
                println!("  {}) {}", j + 1, employees_param_copy_3[j]);
                j+=1;
            }
            print!("\n");
        
            let mut choice_employee:String = String::new();
            let mut choice_employee_int:i32 = 0;
        
            let mut smallest_possible_value_employees:usize = 1;
            let mut greatest_possible_value_employees:usize = employees_param_copy_3.len();
        
            while choice_employee_int < smallest_possible_value_employees as i32 || choice_employee_int > greatest_possible_value_employees as i32
            {
                choice_employee = String::from("");
                choice_employee_int = 0;
        
                warning_message();
                print!("Which employee would you like to add to the department of {}? ", vector_departments[(choice_to_integer - 1) as usize]);
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut choice_employee).expect("Failed to read line");
                choice_employee_int = choice_employee.trim().parse().unwrap();
        
                let mut k:usize = 0;
                while k < employees_param_copy_3.len()
                {
                    if choice_employee_int == (k as i32) + 1
                    {
                        println!("Employee found");
                        break;
                    } else if choice_employee_int != (k as i32) + 1
                    {
                        if k == employees_param_copy_3.len() - 1
                        {
                            println!("This employee doesn't exist");
                        }
                    }
                    k+=1;
                }
            }
        
            let chosen_department:String = String::from(vector_departments[(choice_to_integer - 1) as usize].clone());
            let chosen_employee:String = String::from(employees_param_copy_3[(choice_employee_int - 1) as usize].clone());
            /*
            help: consider creating a fresh reborrow of `company_hashmap_param` here
                |
            204 |     for (key, value) in &mut *company_hashmap_param
            */
        
            insert_str_in_vectorstr_in_hashmap(company_hashmap_param, &chosen_department, &chosen_employee);
        
            println!("Adding {} to the department of {}.", employees_param_copy_3[(choice_employee_int - 1) as usize], vector_departments[(choice_to_integer - 1) as usize]);
        
            if let Some(index) = employees_param_copy_3.iter().position(|value| *value == chosen_employee)
            {
                employees_param_copy_3.remove(index);
                // This solution: https://users.rust-lang.org/t/finding-and-removing-an-element-in-a-vec/42166
            }
        
            for (key, value) in &mut *company_hashmap_param
            {
                for name in value
                {
                    println!("\t{name} was added to the department of {key}!");
                }
            }
            
            if employees_param_copy_3.len() > 0
            {
                println!("  Here is the list of employees:");
                let mut j:usize = 0;
                while j < employees_param_copy_3.len()
                {
                    println!("  {}) {}", j + 1, employees_param_copy_3[j]);
                    j+=1;
                }
            }
    
            let mut choice_continue_program:String = String::new();
            let mut choice_continue_program_int:i32 = 0;
        
            let smallest_possible_choice_continue:u8 = 1;
            let greatest_possible_choice_continue:u8 = 3;
        
            while choice_continue_program_int < smallest_possible_choice_continue as i32 || choice_continue_program_int > greatest_possible_choice_continue as i32
            {
                if employees_param_copy_3.len() < 1 && choice_continue_program_int != 3
                {
                    println!("===>There are no more employees to add. This function will now terminate.");
                    user_wants_add_employee_same_department = false;
                    user_doesnt_want_quit = false;
                    break;
                }

                choice_continue_program = String::from("");
                choice_continue_program_int = 0;
        
                warning_message();
                println!("Would you like to:\n1) Add more employees to the department of {}\n\
                2) Add another employee to a different department\n\
                or 3) Quit the program?", &chosen_department);
                print!("=> Please type in your choice: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut choice_continue_program).expect("Failed to read line");
                choice_continue_program_int = choice_continue_program.trim().parse().unwrap();
        
                println!("You've chosen {}", choice_continue_program_int);
                
                if choice_continue_program_int == 1
                {
                    println!("Then let's continue to add more employees to the department of {}", &chosen_department);
                    break;
                } else if choice_continue_program_int == 2
                {
                    user_wants_add_employee_same_department = false;
                    break;
                } else if choice_continue_program_int == 3
                {
                    println!("User chose 3!!");
                    user_wants_add_employee_same_department = false;
                    user_doesnt_want_quit = false;
                    break;
                } else
                {
                    println!("Please type in a valid input.");
                }
                //Replace these if and else-ifs with match
        }
    }
}

fn insert_str_in_vectorstr_in_hashmap(hashmap_param:&mut HashMap<String, Vec<String>>,
        chosen_key:&String, str_to_add_param:&String)
{
    for (key, vector_value) in hashmap_param
    {
        if string_compare(&chosen_key, key)
        {
            vector_value.push(str_to_add_param.clone());
        }
    }
}

fn string_compare(string_1:&str, string_2:&str) -> bool
{
    if string_1.len() != string_2.len()
    {
        return false;
    }

    let mut i:usize = 0;
    while i < string_1.len()
    {
        let char_to_check_str_1:char = string_1.chars().nth(i).unwrap();

        let mut j:usize = 0;
        while j < string_2.len()
        {
            let char_to_check_str_2:char = string_2.chars().nth(j).unwrap();

            if char_to_check_str_1 != char_to_check_str_2 && i == j
            {
                return false;
            }

            j+=1;
        }
        i+=1;
    }

    return true;
}

fn warning_message()
{
    println!("\nWARNING! Please make sure to only type in INTEGER numbers. Don't type decimal numbers, \
    whitespaces, letters or special characters like !, %, > or ). Otherwise, the program will crash and you'll \
    have to restart.\n");
}

fn print_vector_ref_str(vector_param:&Vec<&str>)
{
    for word in vector_param
    {
        println!("{word}");
    }
}

fn print_vector_str(vector_param:&Vec<String>)
{
    for word in vector_param
    {
        println!("{word}");
    }
}

// Function to compare two strings
fn compare_strs(first_str:&str, second_str:&str) -> bool
{
    // Returns true if the characters in the same index of both strings are different, and
    // the numeric equivalent of the ASCII value of the said character in the first_str is
    // greater than its counterpart in the second_str
    let mut i:usize = 0;
    while i < first_str.len()
    {
        let char_to_comp_1:Option<char> = first_str.chars().nth(i);
        let char_to_comp_1_unwraped:char = char_to_comp_1.unwrap();

        let mut j:usize = 0;
        while j < second_str.len()
        {
            let char_to_comp_2:Option<char> = second_str.chars().nth(j);
            let char_to_comp_2_unwraped:char = char_to_comp_2.unwrap();

            if (char_to_comp_1_unwraped as u8) > (char_to_comp_2_unwraped as u8) && i == j
            {
                return true;
            } else if (char_to_comp_1_unwraped as u8) < (char_to_comp_2_unwraped as u8) && i == j
            {
                return false;
            };
            j+=1;
        }
        i+=1;
    }

    return false;
}

// Function to sort strings in a List/Vector alphabetically
fn sort_strs_in_two_vectors(vector_strs:&mut Vec<String>, original_vector:&mut Vec<String>)
{
    let mut i:usize = 0;
    while i < vector_strs.len()
    {
        let mut first_char_1:Option<char> = vector_strs[i].chars().nth(0);
        let mut first_char_1_unwraped:char = first_char_1.unwrap();

        let mut j:usize = 0;
        while j < vector_strs.len()
        {
            let first_char_2:Option<char> = vector_strs[j].chars().nth(0);
            let first_char_2_unwraped:char = first_char_2.unwrap();

            if (first_char_1_unwraped as u8) > (first_char_2_unwraped as u8) && i < j
            {
                let temporary:String = vector_strs[i].to_string();
                vector_strs[i] = vector_strs[j].to_string();
                vector_strs[j] = temporary;

                let temp_original_vec:String = original_vector[i].clone();
                original_vector[i] = original_vector[j].clone();
                original_vector[j] = temp_original_vec.clone();

                first_char_1 = vector_strs[i].chars().nth(0);
                first_char_1_unwraped = first_char_1.unwrap();
            } else if (first_char_1_unwraped as u8) == (first_char_2_unwraped as u8) && i < j
            {
                let first_and_second_strs_should_swap:bool = compare_strs(&vector_strs[i], &vector_strs[j]);

                if first_and_second_strs_should_swap
                {
                    let temporary:String = vector_strs[i].to_string();
                    vector_strs[i] = vector_strs[j].to_string();
                    vector_strs[j] = temporary;

                    let temp_original_vec:String = original_vector[i].clone();
                    original_vector[i] = original_vector[j].clone();
                    original_vector[j] = temp_original_vec.clone();
                }
            } else if (first_char_1_unwraped as u8) < (first_char_2_unwraped as u8) && i > j
            {
                let temporary:String = vector_strs[i].to_string();
                vector_strs[i] = vector_strs[j].to_string();
                vector_strs[j] = temporary;

                let temp_original_vec:String = original_vector[i].clone();
                original_vector[i] = original_vector[j].clone();
                original_vector[j] = temp_original_vec.clone();

                first_char_1 = vector_strs[i].chars().nth(0);
                first_char_1_unwraped = first_char_1.unwrap();
            }
            
            j+=1;
        }
        i+=1;
    }
}

//Function to copy strings from a string vector to a temporary string vector
fn copy_strs_vectors(vector_to_be_copied:&mut Vec<String>) -> Vec<String>
{
    let mut output_vector:Vec<String> = Vec::new();
    let mut i:usize = 0;
    while i < vector_to_be_copied.len()
    {
        let mut string_to_be_pushed:String= vector_to_be_copied[i].to_string().clone();
        string_to_be_pushed = transform_word_to_lower(&mut string_to_be_pushed);
        output_vector.push(string_to_be_pushed.clone());

        i+=1;
    };

    output_vector
}

// Function to transform the words of a string of vectors into lowecase:
fn transform_word_to_lower(word:&mut String) -> String
{
    let mut word_to_return:String = word.to_string();

    const SMALLEST_UPPER_ASCII:u8 = 65;
    const BIGGEST_UPPER_ASCII:u8 = 90; 
    const DIFF_LOWER_UPPER_ASCII:u8 = 32;

    let mut char_to_change:char;
    let mut i:usize = 0;
    while i < word.len()
    {
        char_to_change = word.chars().nth(i).unwrap();

        if char_to_change as u8 >= SMALLEST_UPPER_ASCII && char_to_change as u8 <= BIGGEST_UPPER_ASCII
        {
            char_to_change = ((char_to_change as u8) + DIFF_LOWER_UPPER_ASCII) as char;
            let current_char_to_str:String = char_to_change.to_string();
            word_to_return.replace_range(i..i+1, &current_char_to_str);
        }
        i+=1;
    }
    word_to_return
}