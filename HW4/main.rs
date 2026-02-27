// solution for Baghers HomeWork HW4
fn main() {
    println!("list of numbers:");
    let mut input_2: String = String::new();

    let _ = std::io::stdin().read_line(&mut input_2);

    let mut angles = input_2.split(' ');

    let first_ang: &str = angles.next().expect("error");
    let first: u32 = first_ang.trim().parse().expect("first input is not uint!");
    let second_ang: &str = angles.next().expect("error");
    let second: u32 = second_ang.trim().parse().expect("second input is not uint!");
    let third_ang: &str = angles.next().expect("error");
    let third: u32 = third_ang.trim().parse().expect("third input is not uint!");

    let first_condition = first + second + third == 180;
    let second_condition: bool = (first > 0) & (second > 0) & (third > 0);

    if first_condition & second_condition {
        println!("Yes");
    } else {
        println!("No")
    }
}