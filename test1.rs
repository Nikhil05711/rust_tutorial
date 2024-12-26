// This function demonstrates ownership and string manipulation in Rust
pub fn test_ownership() {
    // Calling the get_string function, which returns a String
    let s1: String = get_string();
    // Printing the string returned by get_string
    println!("Hi : {}", s1);

    // Creating a new String instance
    let s2: String = String::from("Nikhil Chauhan");
    // Passing the string to send_get_string and getting it back
    let s3: String = send_get_string(s2);
    // Printing the string returned by send_get_string
    println!("Received string: {}", s3);
}

// This function returns a String containing the message "Merry Christmas"
fn get_string() -> String {
    // Creating a new String
    let new_string = String::from("Merry Christmas");
    // Returning the new string
    return new_string;
}

// This function takes ownership of a String, processes it, and returns it
fn send_get_string(received_string: String) -> String {
    // Returning the received string (nothing is changed)
    return received_string;
}
