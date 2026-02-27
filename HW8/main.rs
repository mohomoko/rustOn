fn main() {
    let a = 1.0;
    let b = 2.5;
    let c = 3.33;

    let result = sum_3(a, b, c);
    println!("The sum of {}, {}, {} is {}", a, b, c, result);
}

fn sum_3(mut x:f32, mut y:f32, mut z:f32) -> f32 {
    x + y + z
}