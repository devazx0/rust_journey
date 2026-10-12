fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    // SHADOWING
    let y = 30;
    let y = 30 + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}")
    }

    println!("the value of y in the outer scope is {y}")

}
