fn main() {
    //scalar type represents a single value. Rust has four primary scalar types: integers,
    //floating-point numbers, Booleans, and characters
	
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32    
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

	//Booleans are one byte in size

    let t = true;

    let f: bool = false; // with explicit type annotation

	//char type is most primitive alphabetic type
	//we specify char literals with single quotes, as opposed to string literals, which use double quotes
	//four bytes so can represent ASCII, CJK, emoji, etc.	

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

	//Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

	//We create a tuple by writing a comma-separated list of values inside parentheses
	
	let tup: (i32, f64, u8) = (500, 6.4, 1);

}
