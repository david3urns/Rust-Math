/*
===================================================================
Math quiz program developed by David Burns (david3urns@gmail.com)
This is based off a python script I wrote, using it as a tool to
help get my kids interested in programming while creating a program
that is also educational. 
Please address any bugs/issues to the email address above!
===================================================================
*/



use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write, Read};
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};


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
//starts the main menu
    math_quiz_menu()
}


//function to sleep for x
fn sleep_time(sleep_time_input: u64){
    sleep(Duration::from_secs(sleep_time_input));
}


//function to clear the terminal screen
fn clear_screen(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }


// function to create a random number from /dev/urandom
fn random_generator() -> u32 {
    let mut f = File::open("/dev/urandom").unwrap();
    let mut buf = [0; 4];
    f.read_exact(&mut buf).unwrap();
    let random_number =u32::from_ne_bytes(buf);
    random_number % 100
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


//arithmetic Functions


//addition
fn addition(){
    loop{
        clear_screen();
        banner("Addition");

        //generate the random numbers:
        let rand_num1 = random_generator();
        let rand_num2 = random_generator();

                
        //prints out the question and asks for an answer from the user:
        let answer_int: u32 = loop {
            print!("What is {:?} + {:?}? ", rand_num1, rand_num2);
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();

            match answer.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input, please enter a vaild interger.");
                    continue;
                }
            }
        };


        //perform the calculations for the answer


        //if correct answer is given:
        if rand_num1 + rand_num2 == answer_int {
            println!("You are correct!");
            sleep_time(3);
            correct_count();
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
        }


        //if incorrect answer is given:
        else {
            println!("I got a different answer, please try again.");
            sleep_time(3);
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
        }


        //run the menu to repeat, menu, or exit
        repeat_menu("1");
    }   
    
}


//subtraction function
fn subtraction(){
    loop{
        clear_screen();
        banner("Subtraction");

        //generate the random numbers:
        let mut rand_num1 = random_generator();
        let mut rand_num2 = random_generator();

        //while statement makes sure second number is less than or equal to the first (for subtraction and division)
        while rand_num2 >= rand_num1{
            rand_num1 = random_generator();
            rand_num2 = random_generator();
        };
        
        //generates the problem with random numbers and asks the user for the answer:
        let answer_int: u32 = loop {
            print!("What is {:?} - {:?}? ", rand_num1, rand_num2);
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();

            match answer.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input, please enter a vaild interger.");
                    continue;
                }
            }
        };


        //perform the calculations for the answer

        //if the correct answer is given:
        if rand_num1 - rand_num2 == answer_int {
            println!("You are correct!");
            sleep_time(3);
            correct_count();
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
        }

        //if the incorrect answer is given:
        else {
            println!("I got a different answer, please try again.");
            sleep_time(3);
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
        }

    //runs the repeat menu function
    repeat_menu("2");
    } 
}


//multiplication:
fn multiplication(){
     loop{
        clear_screen();
        banner("Multiplication");

        //generate the random numbers:
        let rand_num1 = random_generator();
        let rand_num2 = random_generator();

                
        //generates the problem with random numbers and asks the user for an answer:
        let answer_int: u32 = loop {
            print!("What is {:?} * {:?}? ", rand_num1, rand_num2);
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();

            match answer.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input, please enter a vaild interger.");
                    continue;
                }
            }
        };


        //perform the calculations for the answer

        //if the correct answer is given:
        if rand_num1 + rand_num2 == answer_int {
            println!("You are correct!");
            sleep_time(3);
            correct_count();
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
        }


        //if the incorrect answer is given:
        else {
            println!("I got a different answer, please try again.");
            sleep_time(3);
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
        }


        //run the menu to repeat, menu, or exit
        repeat_menu("3");
    }
}


//division
fn division(){
     loop{
        clear_screen();
        banner("Division");

        //generate the random numbers:
        let mut rand_num1 = random_generator();
        let mut rand_num2 = random_generator();

        //while statement makes sure second number is less than or equal to the first (for subtraction and division)
        while rand_num2 >= rand_num1{
            rand_num1 = random_generator();
            rand_num2 = random_generator();
        };
        
        
        //generates the question with two random numbers and asks for the answer:
        let answer_int: u32 = loop {
            print!("What is {:?} / {:?}? ", rand_num1, rand_num2);
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();

            match answer.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input, please enter a vaild interger.");
                    continue;
                }
            }
        };

        //remainder input
        let remainder_answer_int: u32 = loop {
            print!("What is the remainder, enter 0 for no remainder? ");
            io::stdout().flush().unwrap();

            let mut remainder = String::new();
            io::stdin().read_line(&mut remainder).unwrap();

            match remainder.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input, please enter a vaild interger.");
                    continue;
                }
            }
        };


        //perform the calculations for the answer
        let remainder_int = rand_num1 % rand_num2;

        //if the correct answer is given:
        if answer_int == rand_num1 / rand_num2 && remainder_answer_int == remainder_int {
            println!("You are correct!");
            sleep_time(3);
            correct_count();
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
        }


        //if the incorrect answer is given:
        else {
            println!("I got a different answer, please try again.");
            sleep_time(3);
            total_count();
            clear_screen();
            print_progress();
            sleep_time(3);
        }


        //run the menu to repeat, menu, or exit
        repeat_menu("4");
    }
}


//main menu function:
fn math_quiz_menu(){
    loop {
        clear_screen();
        banner("Math Quiz");
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


//menu to give the user the option to repeat the same type of question, return to the main menu, or exit
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
        math_quiz_menu();
        std::process::exit(0);
    }
    else if repeat_choice_int == 3 {
        clear_screen();
        println!("Thank you for playing!");
        sleep_time(3);
        std::process::exit(0); 
    }   
}