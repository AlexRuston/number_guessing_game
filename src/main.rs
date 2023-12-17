use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;
use std::io;
use std::cmp::Ordering;

fn main() {
    
    // introduction text
    let intro_string = "Welcome!\nAre you as excited as I am?\nI doubt it.\nWell, lets get to it.\nFirst, on a scale of 1 to 10, how confident are you feeling?\n";

    let speed: u64 = 20;

    slow_print(intro_string, speed);

    // here we're going to let the user pick the difficulty, with a max value of 10
    // if the user picks a number above 10, we will loop around until the pick a number in range
    // set the difficulty variable -> this is later used as the top end of a random number generator
    let mut difficulty = 0;
    // we will pick up how many attempts the user takes
    let mut attempts = 0;
    
    // the user gets asked for their input until they pick a valid difficulty level
    loop {
        // increment the attempts count
        attempts = attempts + 1;

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
    if attempts > 5 {
        let attempts_string = format!("Did that really just take you {} attemps to pick a number lower than 10?.\nBless you...", &difficulty);
        slow_print(&attempts_string, speed);
    }

    let difficulty_string = format!("\nWow!\n{}\nYou're brave.\nOkay, lets get to it.", &difficulty);

    slow_print(&difficulty_string, speed);
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
    let difficulty: i64 = difficulty.trim().parse().unwrap();

    // return
    return difficulty;
}
