mod file1;
mod file2;
mod test1;

fn main() {
    file1::function_declaration();
    // Calling the test_ownership function from test1 module
    test1::test_ownership();
    // Declaration of an integer variable `num` with type `u8` and initialization
    file2::issue_ownership_transfer();
    let num: u8 = 250;
    // Printing the value of `num` to the console
    println!("The value inside the num is {}", num);
    println!(); // Adds an empty line for spacing after printing the first value

    // Calling the `abc` function
    abc();
}

fn abc() {
    // Declaration of a mutable integer variable `num` with type `u8` and initialization
    let mut num: u8 = 250;

    // Printing the value of `num` to the console
    print!("The value inside the num is {}", num);
    println!(); // Adds an empty line after the first print statement

    // Modifying the value of `num`
    num = 99;

    // Printing the updated value of `num` to the console
    print!("The value inside the num is {}", num);
    println!(); // Adds an empty line after printing the second value

    // Calling the `abcd` function
    abcd();
}

fn abcd() {
    // &str is a string slice that is immutable and typically stored in read-only memory (e.g., the program's binary or rodata section).
    // Its length cannot be changed, even if declared with 'mut'.
    let string_literal = "Hii, Nikhil Chauhan";
    println!("The value of string literal is {}", string_literal);

    // String is a heap-allocated, growable, and mutable string type in Rust.
    // It is dynamically allocated and can change its length.
    let mut string_literal1: String = String::from("Hii, Nikhil Chauhan,");
    string_literal1.push_str(" Whats'upp.");
    println!("The value of string literal is {}", string_literal1);

    tuple_and_destructuring();
}

fn tuple_and_destructuring() {
    // Creating a tuple to store employee information.
    // A tuple can hold different types of data.
    // In this case, we are storing a string, an unsigned 8-bit integer, and a boolean.
    // The type of this tuple is (&str, u8, bool)
    let emp_info: (&str, u8, bool) = ("Nikhil Chauhan", 25, true);

    // Accessing the elements of the tuple using dot notation.
    // Tuples use zero-based indexing to access elements.
    // emp_info.0 gives the first element, emp_info.1 gives the second, and emp_info.2 gives the third.
    let emp_name = emp_info.0; // Accessing the employee's name (first element in the tuple)
    let emp_age: u8 = emp_info.1; // Accessing the employee's age (second element in the tuple)
    let emp_available = emp_info.2; // Accessing the employee's availability status (third element in the tuple)

    // Printing the employee's details by using the variables.
    println!(
        "The Name of employee is {}, The age is {}, is He available: {}",
        emp_name, emp_age, emp_available
    );

    // Destructuring the tuple to directly extract values into variables.
    // In destructuring, the values from the tuple are unpacked and assigned to individual variables.
    // This is an alternative to accessing tuple elements via dot notation.
    let (employee_name, employee_age, employee_available) = emp_info;

    // Printing the employee's details after destructuring.
    // This is similar to the previous print statement but using destructured values.
    println!(
        "The Name of employee is {}, The age is {}, is He available: {}",
        employee_name, employee_age, employee_available
    );
}
