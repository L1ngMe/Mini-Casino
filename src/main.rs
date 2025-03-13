use std::io;
mod casino;



fn main() {
    println!("List of commands:\n   play <bet>, \n   balance, \n   exit");
    let mut balance: i32 = 1000;
    let mut input: String = String::new();
    
    loop {
        input.clear();
        

        io::stdin()
            .read_line(&mut input)
            .expect("Error");

        let args: Vec<&str> = input.trim().split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        match args.as_slice() {
            ["play", _] => {
                let bet: i32 = match args[1].parse::<i32>() {
                    Ok(num) => {num},
                    Err(_) => {
                        println!("play <bet>");
                        continue;
                    },
                };

                let win: i32 = casino::play(bet);
                if win == 0 {
                    println!("You lose(");
                    balance -= bet;
                } else {
                    println!("Your win: {}", win);
                    balance += win;
                }
                println!("Your balance: {}", balance);
                continue;
            },
            ["balance"] => println!("Balance: {}", balance),
            ["exit"] => break,
            ["help"] => println!("List of commands:\n   play <bet>, \n   balance, \n   exit"),
            _ => println!("Unknown command")
        }
        
    }
}
