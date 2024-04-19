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
    println!("Press 1 -> Celcius to Fahrenheit Converter");
    println!("Press 2 -> Fahrenheit to Celcius Converter");

    let mut option = String::new();
    io::stdin().read_line(&mut option)
        .expect("Failed to read line");
    let option : u8 = option.trim().parse()
        .expect("Failed to parse");

    if option == 1 {
        println!("Please enter temp in celcius");
        let mut cel = String::new();
        io::stdin().read_line(&mut cel)
            .expect("Failed to read line");

        let cel : f32 = cel.trim().parse()
            .expect("Failed to parse");

        let far = (cel * (9.0/5.0) ) + 32.0 ;

        println!("Temperature in Fahrenheit is : {}", far);
    }else{
        println!("Please enter temp in Fahrenheit");
        let mut f = String::new();
        io::stdin().read_line(&mut f)
            .expect("Failed to read line");

        let f : f32 = f.trim().parse()
            .expect("Failed to parse");

        let c : f32 = ( f - 32.0) * (5.0/9.0);

        println!("Temperature in Celcius is : {}", c);
    }
}

fn fibonacci() {
    println!("Welcome to Fibonacci nubers!");
    println!("Enter a number from 1 to 100 ");
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Failed to read line");

    let num :u8 = num.trim().parse()
        .expect("Failed to parse number");
    
    let mut  a = 0;
    let mut b = 1;
    let mut c = 1;

    let mut i = 1;
    while i < num {
        c = a + b;
        a = b;
        b = c;
        i = i+1;
    }
    println!("{}th fib number is : {}", num, c);

}

fn christmasCarol() {
    println!("Welcome to Christmas carol");

    let a = ["first","second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    let gifts = ["A partridge in a pear tree.", "Two turtle doves", "Three French hens", "Four calling birds", "Five gold rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    let mut song = String::new();

    for i in 0..a.len() {
        
        println!("On the {} day of Christmas, my true love sent to me", a[i]);
        
        if i == 0 {
            song = "A partridge in a pear tree.".to_string();
            println!("{}",song);
            song = "And a partiride in a pear tree.".to_string();
        }else {
            song = format!("{}, \n{}", gifts[i], song);
            println!("{}",song);
        }
        println!("\n");
    }

}
