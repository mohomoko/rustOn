// Debug

/*  
 * 1. Run the code and see the error message.
 * 2. Install `C/C++ (microsoft.com) (594)` extension in VSCode to get better error messages.
 * 3. Use Debug to find the error.
 *   - Set a breakpoint on the line `let mut number : u8 = input.trim().parse().expect("Type uint number");`
 *   - Run the code in Debug mode and input a number, e.g. `5`.
 * 4. Check the value of `input` and `number` in the Debug console.
 * 
 */
fn main() {
    let mut input: String = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    let mut number : u8 = input.trim().parse().expect("Type uint number");

    print!("W");
    while number > 0 {
        print!("o");
        number -= 1
    }
    println!("w!");
}