use std::io;

fn main() {
    println!("Hello, world!");
    let unit = menu();
    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    let value: u32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let v: u32;
    if unit == 1 {
        v = value * 9/5 + 32;
    }
    
    else {
        v = value;
    }

    println!("{}", v);
 
    
}

fn menu()-> u32 {
    println!("Select what's your unit ! 1 / 2 / 3");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("{}", guess);
    let unit = guess;
    let unit: u32 = match unit.trim().parse() {
        Ok(num) => num,
        Err(_) => return 1,
    };
    return unit
}

