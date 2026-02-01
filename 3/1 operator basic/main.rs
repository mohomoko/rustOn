fn main() {
    let a = 10;
    let b = 20;

    let mut result = a + b;
    println!("Initial result: {result}");

    result = a - b;
    println!("After subtraction: {result}");

    result = a * b;
    println!("After multiplication: {result}");

    result = a / b;
    println!("After division: {result}");

    result = b % a;
    println!("After modulus: {result}");

    let c: f32 = 11.0;
    let d: f32 = 23.;

    let result = c / d;
    println!("Floating-point division result: {result}");

    // e = e + 5 is equal to e += 5
    let mut e = 5;
    e += 5;
    println!("After e += 5: {e}");
}
