use std::io::{self, Write};

fn main() {
    println!("\nWelcome to our HangMan Game.. \n");
    let answer_string: String = String::from("welcome");
    let mut wrong_attempt_count = 0;

    let mut answers: Vec<String> = Vec::new();

    loop {
        let mut _input = String::new();

        for i in answer_string.chars() {
            if answers.contains(&i.to_string()) {
                print!("{}", i.to_uppercase());
            } else {
                print!("_");
            }
        }

        println!("\n");

        hang_man(wrong_attempt_count);

        if wrong_attempt_count >= 7 {
            println!("Man Hanged 🙁");
            break;
        }

        if answers.len() == answer_string.len() {
            println!("\n CONGRATS. You WON!  🎉🎉🎉");
            break;
        }


        print!("\nGuess a letter: ");

        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut _input)
            .expect("Error reading from STDIN");

        if !_input.trim().is_empty() && answer_string.contains(&_input.trim()) {
            println!("asasas");
            answers.push(_input.trim().to_string());
        } else {
            wrong_attempt_count += 1;
        }
    }
}

fn hang_man(step: i8) {
    println!("{}", "-".repeat(9));

    for i in 0..6 {
        let mut line: String = String::new();
        line.push_str("|");

        if i == 0 {
            line.push_str(&" ".repeat(7));
            line.push_str(&"|");
        }

        if step >= 1 && i == 1 {
            line.push_str(&" ".repeat(7));
            line.push_str(&"o");
        }

        if i == 2 {
            if step == 2 {
                line.push_str(&" ".repeat(7));
                line.push_str(&"|");
            } else if step == 3 {
                line.push_str(&" ".repeat(6));
                line.push_str(&r"\|");
            } else if step >= 4 {
                line.push_str(&" ".repeat(6));
                line.push_str(&r"\|/");
            }
        }

        if i == 3 {
            if step >= 5 {
                line.push_str(&" ".repeat(7));
                line.push_str(&"|");
            }
        }

        if i == 4 {
            if step == 6 {
                line.push_str(&" ".repeat(6));
                line.push_str(&"/");
            } else if step == 7 {
                line.push_str(&" ".repeat(6));
                line.push_str(&r"/ \");
            }
        }

        println!("{}", line);
    }

    println!("");
}
