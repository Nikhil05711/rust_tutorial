// src/file1.rs

// <<<<<<<<<<<<<<<  Module 3: FUNCTION DECLARATION  >>>>>>>>>>>>>>>>

pub fn function_declaration() {
    let num1: u8 = 10;
    let num2: u8 = 20;
    let result: u8 = add(num1, num2); // Calling the `add` function
    println!("The result for two numbers is {}", result);
    scope();
    static_memory();
    dynamic_memory();
}

// Corrected the return type `-> u8` in the add function
fn add(item1: u8, item2: u8) -> u8 {
    return item1 + item2; // Returning the sum of item1 and item2
}

// <<<<<<<<<<<<<<<  Module 4: OWNERSHIP  >>>>>>>>>>>>>>>>

// Why is ownership needed in Rust?
// Ownership is needed because it eliminates the disadvantages of both
// traditional manual memory management (with control points) and the overhead of
// garbage collectors. Rust's ownership model ensures memory safety and
// avoids runtime costs, providing a more efficient way to manage memory.

// What is the difference between Stack and Heap ?
// Difference between Stack and Heap:
// - The Stack is a region of memory used for storing fixed-size variables and function call information.
//   It follows the Last In, First Out (LIFO) order, meaning data is pushed and popped in a specific order.
//   Stack memory is automatically managed, and variables are allocated and deallocated when they go out of scope.
//
// - The Heap is a region of memory used for storing dynamically-sized data (e.g., data whose size is determined at runtime).
//   Memory allocation and deallocation on the heap are manually controlled or done through a garbage collector in some languages.
//   In Rust, the ownership model ensures that memory is freed once it is no longer needed, preventing memory leaks.

// OwnerShip Rules
// - Each value in a rust has a variable that's called its owner.
//   There can be only one owner at a time.
//   When the owner goes out of scope, the value will be dropped.

// <<<<<<<<<<<<<<<  Module 4.1: SCOPE  >>>>>>>>>>>>>>>>

// Global constant declaration.
// The value 300 is too large for u8, which can only hold values between 0 and 255.
// We change it to a value within that range, so we use u16.
const GLOBAL_CONST: u16 = 200;

fn scope() {
    // Declaring a variable outside of the inner block.
    let outside_variable: u8 = 100;

    // Start of a new block (inner scope).
    {
        // Declaring a variable inside the inner block. It is only accessible within this block.
        let inside_variable: u8 = 120;

        // Accessing the inside_variable inside the block and printing it.
        println!("The inside variable is {}", inside_variable);
        println!(); // Print an empty line for spacing.
    } // `inside_variable` goes out of scope here.

    // Accessing the outside_variable here, as it is in the outer scope.
    println!("The outside variable is {}", outside_variable);
    println!(); // Print an empty line for spacing.

    // Calling another function and passing the outside_variable to it.
    declare_outside_variable(outside_variable);

    // Printing the value of the global constant.
    println!("The global variable is {}", GLOBAL_CONST);
    println!("");
}

// This function takes `outside_variable` as an argument and prints it.
fn declare_outside_variable(outside_variable: u8) {
    println!("Inside declare_outside_variable: {}", outside_variable);
    println!("");
    println!("############################");
    println!("");
}

// <<<<<<<<<<<<<<<  Module 4.2: OWNERSHIP PRACTIAL  >>>>>>>>>>>>>>>>

// Function to demonstrate stack memory allocation
fn static_memory() {
    // Here memory is stack allocation
    let a: u8 = 123; // 'a' is allocated on the stack.
    let b: u8 = a; // 'b' is a copy of 'a' since u8 is a Copy type.

    println!("a is {}", a); // Prints: a is 123
    println!("b is {}", b); // Prints: b is 123
                            // Since 'a' and 'b' are of Copy types, both variables are independent, and no ownership issues arise.
}

// Function to demonstrate dynamic memory allocation (heap allocation) and ownership transfer
fn dynamic_memory() {
    let str1: String = String::from("Hello"); // 'str1' owns the heap-allocated memory for "Hello"

    // Moving ownership from 'str1' to 'str2'. After this, 'str1' is no longer valid.
    let str2: String = str1; // Ownership of the heap memory is transferred to 'str2'
    let mut str3: String = str2; // Ownership of the heap memory is transferred to 'str3'

    // Use push_str to add " world" to str3
    str3.push_str(" world"); // Adds " world" to the existing string

    // 'str1' is no longer valid after the move, and accessing it here would cause a compile-time error.
    // println!("str1 is {}", str1); // Uncommenting this line would result in a compile-time error.

    println!("str3 is {}", str3); // Prints: str2 is Hello
                                  // 'str2' is now the owner of the heap memory, and once it's out of scope, the memory will be deallocated.
}

// <<<<<<<<<<<<<<<  Module 4.3: OWNERSHIP AND FUNCTION  >>>>>>>>>>>>>>>>
