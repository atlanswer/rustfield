fn main() {
    // Immutable vs. mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // Shadowing
    {
        let x = 7;
        println!("The value of x in the inner scope is {x}");
    }
    // Constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // Floating point numbers
    let y: f32 = 3.0;
    // Boolean
    let t: bool = true;
    // Characters
    let thumbs_up_emoji: char = '👍';
    // Tuple
    let tup: (i32, u32, f32) = (-1, 1, 2.0);
    // Array
    let a = [1, 2, 3, 4, 5];
    let b: [u32; 5] = [1, 2, 3, 4, 5];
    let c = [3, 5];
}
