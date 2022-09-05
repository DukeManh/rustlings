// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

fn shadow(x: &u32) -> u32 {
    let x = x * 2;
    x
}

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);

    let number =  shadow(&number);
    println!("Number is : {}", number);
}
