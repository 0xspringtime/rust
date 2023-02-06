fn main() {
    //A reference is like a pointer in that it’s an address we can follow to access the data stored
    //at that address; that data is owned by some other variable. Unlike a pointer, a reference is
    //guaranteed to point to a valid value of a particular type for the life of that reference
	let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
//ampersands represent references, allowing us to refer a value without taking ownership of it
fn calculate_length(s: &String) -> usize {
    s.len()
}//s goes out of scope but is not dropped because it does not have ownership
//opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *

//example with mutable references:
//fn main() {
//    let mut s = String::from("hello");
//
//    change(&mut s);
//}
//
//fn change(some_string: &mut String) {
//    some_string.push_str(", world");
//}
//code that attempts to have two mutable references to a variable will fail
