fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const TEST_TEST: u32 = 5;
    println!("The value of x is: {}", TEST_TEST);

    let tup = (500, 6.4, 'A');
    let (_x, _y, _z) = tup;

    println!("The value of tup is: {}", tup.2);

    let array: [u32; 6] = [1, 2, 8, 63, 547, 420];

    println!("Array element: {}", array[5]);
}
