use rand::Rng;
use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;
use std::io;
use std::cmp::Ordering;

fn main() {

    // how fast do we want the text to default typing to
    let speed: u64 = 30;

    // introduction text
    slow_print("Welcome!\nAre you as excited as I am?\nI doubt it.\nWell, lets get to it.\nFirst, on a scale of 1 to 10, how confident are you feeling?\n", speed);

    // here we're going to let the user pick the difficulty, with a max value of 10
    // if the user picks a number above 10, we will loop around until the pick a number in range
    // set the difficulty variable -> this is later used as the top end of a random number generator
    let mut difficulty = 0;
    // we will pick up how many attempts the user takes
    let mut attempts_diff = 0;
    
    // the user gets asked for their input until they pick a valid difficulty level
    loop {
        // increment the attempts count
        attempts_diff = attempts_diff + 1;

        // assign the difficulty variable
        // we've extracted this to its own function
        difficulty = get_difficulty();
        
        // check the users input is below 10
        match difficulty.cmp(&10) {
            // less than 10 means exit loop
            Ordering::Less => {
                break
            },
            // this is effectively what's causing the loop to stick
            Ordering::Greater => println!("I said between 1 and 10...\nLets try again..."),
            // equal to 10 also means exit loop
            Ordering::Equal => {
                break;
            }
        }
    }

    // for added flavour, lets belittle the user if the take a long time to input a number 10 or below
    if attempts_diff > 5 {
        let attempts_string = format!("Did that really just take you {} attemps to pick a number lower than 10?.\nBless you...", difficulty);
        slow_print(&attempts_string, speed);
    }

    let difficulty_string = format!("\nWow!\n{}\nYou're brave.\nOkay, lets get to it.\n", difficulty);
    slow_print(&difficulty_string, speed);

    slow_print("Beep...\nHmm...\nBoop...\n", 90);

    // assign a number between 1 and the difficulty level the user chose -> this needs to be a u32 in order for .cmp to accept it
    let computer_number = rand::thread_rng().gen_range(1..=difficulty) as u32;

    slow_print("Okay, I've thought of a number...\nCan you guess what it is?\n", speed);
    
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&computer_number) {
            Ordering::Less => slow_print("Too low there buddy...\nTry again\n", speed),
            Ordering::Greater => slow_print("Too high this time...\nTry again\n", speed),
            Ordering::Equal => {
                let winner_string = format!("\nWow!\n{}\nYou got it.\nGood work!", &computer_number);
                slow_print(&winner_string, speed);
                break;
            }
        }
    }
    
}

// text = string to print
// speed = int sleep duration
fn slow_print(text: &str, speed: u64){

    // loop characters in the string
    for c in text.chars() {
        // print character
        print!("{}", c);
        stdout().flush();
        // delay the next character
        sleep(Duration::from_millis(speed));
    };
}

// creates an input we'll use for the difficulty setting
fn get_difficulty()-> i64 {

    // prepare for user input
    let mut difficulty = String::new();

    // user selects difficulty
    io::stdin()
        .read_line(&mut difficulty)
        .expect("expected to read line");

    // cast to int
    let difficulty: i64 = difficulty.trim().parse::<i64>().expect("That's not a number");

    // return
    return difficulty;
}
