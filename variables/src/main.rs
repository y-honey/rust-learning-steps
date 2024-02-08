fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    const HOUR_IN_SECONDS:u32 = 60 * 60;
    println!("The count of seconds in an hour is {HOUR_IN_SECONDS}");

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x inside inner scope is {x}");
    }
    println!("The value of x outside inner scope is {x}");
}
