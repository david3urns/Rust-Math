use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};

//use std::sync::Mutex;
//use std::process::Command;

//declare static variables for score tracking
static GLOBAL_CORRECT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_TOTAL: AtomicUsize = AtomicUsize::new(0);

//function for tracking and increasing the correct number of questions answered
fn correct_count() {
    GLOBAL_CORRECT.fetch_add(1, Ordering::SeqCst);
}

//function for tracking the total number of problems checked
fn total_count() {
    GLOBAL_TOTAL.fetch_add(1, Ordering::SeqCst);
}

//function to print out the "score" of how many questions answered total versus correct answers
fn print_progress() {
    let global_correct = GLOBAL_CORRECT.load(Ordering::SeqCst);
    let global_total = GLOBAL_TOTAL.load(Ordering::SeqCst);
    let percent_correct = (global_correct as f64 / global_total as f64) * 100.0;
    println!("You have answered a total of {} questions, of which you have answered {} correctly for a score of {:.2}%.", global_total, global_correct, percent_correct);
}

fn main() {
    math_ck_menu()
}

//function to sleep for x
fn sleep_time(sleep_time_input: u64){
    sleep(Duration::from_secs(sleep_time_input));
}

//function to clear the terminal screen
fn clear_screen(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

//function to create a banner for each menu
fn banner(ban_title: &str) {
    let h_border = "═";
    let v_border = "║";
    let tl_corner = "╔";   
    let tr_corner = "╗";
    let bl_corner = "╚";
    let br_corner = "╝";

    //determine the length of the title string
    let title_length = ban_title.len();

    //print the actual box:
    println!("{}{}{}{}{}", tl_corner, h_border, h_border.repeat(title_length), h_border, tr_corner);
    println!("{}{}{}{}{}", v_border, " ", ban_title, " ", v_border);
    println!("{}{}{}{}{}", bl_corner, h_border, h_border.repeat(title_length), h_border, br_corner);


    //future feature to justify and add color to the banner and text
}

//arithmetic functions:

fn addition(){
    loop{
        clear_screen();
        banner("Addition");

        let num_1_int: i32 = loop {
            print!("Please enter the first number of the problem: ");
            io::stdout().flush().unwrap();

            let mut num_1 = String::new();
            io::stdin().read_line(&mut num_1).unwrap();

            match num_1.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input, please enter an interger.");
                    continue;
                }
            }
        };

        let num_2_int: i32 = loop {
            print!("Please enter the second number of the problem: ");
            io::stdout().flush().unwrap();

            let mut num_2 = String::new();
            io::stdin().read_line(&mut num_2).unwrap();

            match num_2.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input, please enter an interger.");
                    continue;
                }
            }
        };

        let answer_int: i32 = loop {
            print!("Please enter your answer: ");
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();

            match answer.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input, please enter an interger.");
                    continue;
                }

            }
        };

        if num_1_int + num_2_int == answer_int {
            println!("You are correct!");
            sleep_time(3);
            correct_count();
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
        }
        else {
            println!("I got a different answer, please try again.");
            sleep_time(3);
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
         
        }

        repeat_menu("1");
    }   
    
}

fn subtraction(){
    clear_screen();
    banner("Subtraction");
    println!("Please enter the first number of the problem: ");
    let mut num_1 = String::new();
    io::stdin()
        .read_line(&mut num_1)
        .expect("Failed to read line");
    let num_1_int: i32 = num_1.trim().parse().expect("Input not an interger");

    println!("Please enter the second number of the problem: ");
    let mut num_2 = String::new();
    io::stdin()
        .read_line(&mut num_2)
        .expect("Failed to read line");
    let num_2_int: i32 = num_2.trim().parse().expect("Input not an interger");

    println!("Please enter your answer: ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line.");
    let answer_int: i32 = answer.trim().parse().expect("Input not an interger");

    //perform the subtraction:
    if num_1_int - num_2_int == answer_int {
        println!("You are correct!");
        sleep_time(3);
        correct_count();
        total_count();
        clear_screen();
        print_progress();
        sleep_time(3);
    }
    else {
        println!("I got a different answer, please try again.");
        sleep_time(3);
        total_count();
        clear_screen();
        print_progress();
        sleep_time(3);

    }
    repeat_menu("2");
}


fn multiplication(){
    clear_screen();
    banner("Multiplication");
    println!("Please enter the first number of the problem: ");
    let mut num_1 = String::new();
    io::stdin()
        .read_line(&mut num_1)
        .expect("Failed to read line");
    let num_1_int: i32 = num_1.trim().parse().expect("Input not an interger");

    println!("Please enter the second number of the problem: ");
    let mut num_2 = String::new();
    io::stdin()
        .read_line(&mut num_2)
        .expect("Failed to read line");
    let num_2_int: i32 = num_2.trim().parse().expect("Input not an interger");

    println!("Please enter your answer: ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line.");
    let answer_int: i32 = answer.trim().parse().expect("Input not an interger");

    //perform the multiplication:
    if num_1_int * num_2_int == answer_int {
        println!("You are correct!");
        sleep_time(3);
        correct_count();
        total_count();
        clear_screen();
        print_progress();
        sleep_time(3);
    }
    else {
        println!("I got a different answer, please try again.");
        sleep_time(3);
        total_count();
        clear_screen();
        print_progress();
        sleep_time(3);
    }
    repeat_menu("3");
}


fn division(){
    clear_screen();
    banner("Division");
    println!("Please enter the first number of the problem: ");
    let mut num_1 = String::new();
    io::stdin()
        .read_line(&mut num_1)
        .expect("Failed to read line");
    let num_1_int: i32 = num_1.trim().parse().expect("Input not an interger");
    //result of this is num_1_int

    println!("Please enter the second number of the problem: ");
    let mut num_2 = String::new();
    io::stdin()
        .read_line(&mut num_2)
        .expect("Failed to read line");
    let num_2_int: i32 = num_2.trim().parse().expect("Input not an interger");
    //result of this is num_2_int

    println!("Please enter your answer: ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line.");
    let answer_int: i32 = answer.trim().parse().expect("Input not an interger");
    //result of this is answer_int
    
    println!("Please enter your remainder, enter zero if no remainder: ");
    let mut rem_answer = String::new();
    io::stdin()
        .read_line(&mut rem_answer)
        .expect("Failed to read line.");
    let rem_answer_int: i32 = rem_answer.trim().parse().expect("Input not an interger");
    let remainder_int: i32 = num_1_int % num_2_int;
    //result of this is rem_answer_int and remainder

    //perform the division:
    if answer_int == num_1_int / num_2_int && rem_answer_int == remainder_int {
        println!("You are correct!");
        sleep_time(3);
        correct_count();
        total_count();
        clear_screen();
        print_progress();
        sleep_time(3);
    }
    else {
        println!("I got a different answer, please try again.");
        sleep_time(3);
        total_count();
        clear_screen();
        print_progress();
        sleep_time(3);
    }
    repeat_menu("4");
}


//main menu function:
fn math_ck_menu(){
    loop {
        clear_screen();
        banner("Math Checker");
        println!("");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");
        println!("");
        print!("Please enter your selection (1-5): ");
        io::stdout().flush().unwrap();
        let mut menu_select = String::new();
        io::stdin().read_line(&mut menu_select).unwrap();

        match menu_select.trim() {
            "1" => addition(),
            "2" => subtraction(),
            "3" => multiplication(),
            "4" => division(),
            "5" => {
                clear_screen();
                println!("Thank you for playing");
                sleep_time(3);
                std::process::exit(0);
            }
            _ => {
                println!("Invalid input, please enter a number between 1 and 5.");
                sleep_time(5);
            }
        }
    }
}

fn repeat_menu(arith_op: &str) {
    println!("Enter 1 to repeat, 2 to return to the main menu, or 3 to exit... ");
    let mut repeat_choice =  String::new();
    io::stdin()
        .read_line(&mut repeat_choice)
        .expect("Failed to read line");
    let repeat_choice_int: i32 = repeat_choice.trim().parse().expect("Input not an interger.");
    if repeat_choice_int == 1 {
        clear_screen();
        if arith_op == "1" {
            addition();
        }
        else if arith_op == "2" {
            subtraction();
        }
        else if arith_op == "3" {
            multiplication();
        }
        else if arith_op == "4" {
            division();
        }
    }
    else if repeat_choice_int == 2 {
        clear_screen();
        math_ck_menu();
        std::process::exit(0);
    }
    else if repeat_choice_int == 3 {
        clear_screen();
        std::process::exit(0); 
    }   
}

