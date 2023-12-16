use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;
use std::io;

fn main() {
    // introduction text
    let intro_string = "Welcome!\nAre you as excited as I am?\nI doubt it.\nWell, lets get to it.\nFirst, on a scale of 1 to 10, how confident are you feeling?\n";

    let speed: u64 = 20;

    slow_print(intro_string, speed);

    let mut difficulty = 0;

    // user selects difficulty
    let mut difficulty_input = String::new();

    io::stdin().read_line(&mut difficulty_input).expect("expected to read line");

    // assign users input to int
    let difficulty: i8 = difficulty_input.trim().parse().unwrap();
    let difficulty_string = format!("\nWow!\n{}\nYou're brave.\nOkay, lets get to it.", difficulty);

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
