fn main() {
    //constants are always immutable, can be declared in any scope
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");
    println!("The value of three hours is: {THREE_HOURS_IN_SECONDS}");
    //shadowing gives x a new value only within its scope
	{
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
