use std::io;

fn main() {
    println!("Assignment 1\n**********************************\n");

    println!("Please Enter: \n 1 for \"Fahrenheit and Celsius Conversion\" \n 2 for \"nth Fibonacci number\"  \n 3 for \"Christmas carol \" \n****************************\n");

    let mut choice = String::new();
    
    //reading input from the console
    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");
    
    //shadowing and parsing the choice
    let choice : u8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("You entered : {}", choice);

    match choice {
        1 => tempConversion(),
        2 => fibonacci(),
        3 => christmasCarol(),
        _ => println!("Thank you!"),
    }
}

fn tempConversion(){
    println!("Welcome to Temperature Conversion!");
}

fn fibonacci() {
    println!("Welcome to Fibonacci nubers!");
}

fn christmasCarol() {
    println!("Welcome to Christmas carol");

}
