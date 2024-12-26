// The function `issue_ownership_transfer` demonstrates ownership transfer and printing in Rust.
pub fn issue_ownership_transfer() {
    // Create a new String object `s1` with the value "Hii_there_is_issues".
    let s1: String = String::from("Hii_there_is_issues");

    // Pass `s1` to the `calculate_length` function, transferring ownership of the string.
    // The function returns a tuple: the original string `s` (now owned by the function) and its length.
    let (s2, len) = calculate_length(s1);

    // The ownership of `s1` is now transferred to `calculate_length`, so it can no longer be used here.
    // `s2` now owns the string, and `len` holds the length of the string.
    println!(
        "The owner of the string is {} and its length is {}", // Prints the string and its length.
        s2, len
    );

    // Call the `clone_method` function to demonstrate cloning a string.
    clone_method();
}

// Function to calculate the length of a string and return both the string and its length as a tuple.
// This function takes ownership of the string, which is why the ownership of `s1` is transferred here.
fn calculate_length(s: String) -> (String, usize) {
    // Calculate the length of the string `s`.
    let length: usize = s.len();

    // Return the string `s` and its length as a tuple.
    return (s, length); // Ownership of `s` is returned back to the calling function.
}

// ANOTHER APPROACH: CLONE METHOD
//
// This section demonstrates the use of `clone()` to make a deep copy of a string.
//
// Clone method is used for creating a deep copy of heap-allocated data (like `String`).
// This method is generally expensive because it involves allocating new memory on the heap.
fn clone_method(){
    // Create a new `String` `s1` with the value "Hello".
    let s1: String = String::from("Hello");

    // Create a clone of `s1` using the `clone()` method, which deep copies the string into a new allocation.
    let s2: String = s1.clone();

    // Both `s1` and `s2` now hold the value "Hello", but they are independent in memory.
    // `clone()` does not transfer ownership, so both variables can be used.
    println!("The value of s1 is {}, and s2 is {}", s1, s2);
    // This prints the values of `s1` and `s2`, which are both "Hello".
    // Since `clone()` was used, `s1` is still valid and can be accessed here.
}
