use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Hash)]
pub enum Department {
    ENGINEERING,
    SALES,
    NOTHING
}

struct Company {
    employees : HashMap<Department, Vec<String>>
}

impl Company {

    /// construct a new Company.  Builds a singleton database
    fn new() -> Company {

        let mut employees = HashMap::new();
        employees.insert(Department::SALES, Vec::new());
        employees.insert(Department::ENGINEERING, Vec::new());

        Company { employees: employees }
    }

    /// break a sentence into multiple parts.  For example, give
    /// a sentence like "this is a string", you will get
    /// a vector containing "this","is","a","string"
    fn tokenize(text: &String) -> Vec<String> {
            let mut return_vector: Vec<String> = Vec::new();
            let mut count = 0;
            let mut current_index = 0;
            for i in text.chars() {
                count += 1;
                if i == ' ' {
                    let item = text[current_index..count-1].to_string();
                    return_vector.push(item);
                    current_index = count;
                }

            }
            let item = text[current_index..count].to_string();
            return_vector.push(item);
            return_vector
    }

    /// processes commands by users.  Currently, the only
    /// command accepted is:
    ///   Add <name> to <engineering or sales>
    fn process_command(&mut self, text: &str) {
        // break the text into tokens
        let tokens: Vec<String> = Company::tokenize(&text.to_string());

        // if the first token is add...
        if &tokens[0].to_uppercase() == "ADD" {
            // take the second token as the name
            let name = &tokens[1];
            // if the third token is "to"
            if &tokens[2].to_uppercase() == "TO" {
                // take the fourth token, find the department name
                let dept: Department;
                match tokens[3].to_lowercase().as_str() {
                    "engineering" => dept = Department::ENGINEERING,
                    "sales" =>       dept = Department::SALES,
                    _ =>             dept = Department::NOTHING,
                }

                // now we know the name and the department, so simply get the
                // proper department, you'll get a vector, and then add the name to
                // that vector
                match self.employees.get_mut(&dept) {
                    Some(database) => database.push(name.to_owned()),
                    None => panic!("aiii ya ya ya ya ya"),
                }
            }
        }
    }


    /// return a list of all the people who are currently affiliated with
    /// a given department.
    fn get_list_of_department_people(&self, dep: Department) -> Vec<String>{
        let result = self.employees.get(&dep);
        match result {
            Some(value) => value.to_vec(),
            None => panic!("omg omg omg omg"),
        }
    }
}





#[cfg(test)]
mod tests {

    use super::*;
    use crate::Department::ENGINEERING;

    // Using a hash map and vectors, create a text interface to allow a
    // user to add employee names to a department in a company. For
    // example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
    // the user retrieve a list of all people in a department or all
    // people in the company by department, sorted alphabetically.

    /// a high-level acceptance test.  will be resolved by
    /// further tests.
    #[test]
    fn test_should_enable_adding_employee_to_department() {
        let mut my_company = Company::new();
        my_company.process_command("Add Sally to Engineering");

        let list: Vec<String> =
            my_company.get_list_of_department_people(ENGINEERING);

        assert_eq!(vec!("Sally"), list);
    }

    #[test]
    fn test_should_enable_adding_another_employee_to_department() {
        let mut my_company = Company::new();
        my_company.process_command("Add Bob to Engineering");

        let list: Vec<String> =
            my_company.get_list_of_department_people(ENGINEERING);

        assert_eq!(vec!("Bob"), list);
    }

    #[test]
    fn test_should_enable_adding_another_employee_to_sales_department() {
        let mut my_company = Company::new();
        my_company.process_command("Add Bob to Sales");

        let list: Vec<String> =
            my_company.get_list_of_department_people(Department::SALES);

        assert_eq!(vec!("Bob"), list);
    }

    #[test]
    fn test_should_enable_adding_two_employees_to_department() {
        let mut my_company = Company::new();
        my_company.process_command("Add Alice to Engineering");
        my_company.process_command("Add Bob to Engineering");

        let list: Vec<String> =
            my_company.get_list_of_department_people(ENGINEERING);

        assert_eq!(vec!("Alice","Bob"), list);
    }

    #[test]
    fn test_should_enable_adding_employees_to_two_departments() {
        let mut my_company = Company::new();
        my_company.process_command("Add Alice to Engineering");
        my_company.process_command("Add Bob to Sales");

        let eng_list: Vec<String> =
            my_company.get_list_of_department_people(Department::ENGINEERING);

        assert_eq!(vec!("Alice"), eng_list);

        let sales_list: Vec<String> =
            my_company.get_list_of_department_people(Department::SALES);

        assert_eq!(vec!("Bob"), sales_list);
    }

    #[test]
    fn test_should_enable_adding_multiple_employees_to_two_departments() {
        let mut my_company = Company::new();
        my_company.process_command("Add Alice to Engineering");
        my_company.process_command("Add Alice to Sales");
        my_company.process_command("Add Bob to Sales");
        my_company.process_command("Add Bob to Engineering");

        let eng_list: Vec<String> =
            my_company.get_list_of_department_people(Department::ENGINEERING);

        assert_eq!(vec!("Alice", "Bob"), eng_list);

        let sales_list: Vec<String> =
            my_company.get_list_of_department_people(Department::SALES);

        assert_eq!(vec!("Alice", "Bob"), sales_list);
    }

    #[test]
    fn test_should_enable_adding_three_employees_to_department() {
        let mut my_company = Company::new();
        my_company.process_command("Add Alice to Engineering");
        my_company.process_command("Add Bob to Engineering");
        my_company.process_command("Add Carol to Engineering");

        let list: Vec<String> =
            my_company.get_list_of_department_people(ENGINEERING);

        assert_eq!(vec!("Alice","Bob", "Carol"), list);
    }

    /// tokenize should take a string and divide it up
    /// using the spaces as delimiters
    #[test]
    fn test_should_tokenize_text() {
        let sentence = String::from("this is text");
        let result = Company::tokenize(&sentence);
        assert_eq!(vec!("this", "is", "text"), result);
    }
}
