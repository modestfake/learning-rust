fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let tup = (500, 6.4, 'A');
    let (_x, _y, _z) = tup;

    println!("The value of tup is: {}", tup.2);

    let array: [u32; 6] = [1, 2, 8, 63, 547, 420];

    println!("Array element: {}", array[5]);
}
