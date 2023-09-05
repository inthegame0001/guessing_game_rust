//Importing what we need. rand is used to generate random numbers, ordering is used to react to what use enters, io(input,output) is used to read what user has typed.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //Creating secret random number
    let _sys_secret_number = rand::thread_rng().gen_range(0..=100);
    //In debugging, we will print secret number and user can see it, but in production, you will need to remove line below.
    println!("system's number: {}", _sys_secret_number);
    //In order to give the user the chance to guess several times, we use a loop.
    loop {
        //user_input has a empty string type inside, but we will replace it with what user has entered.
        let mut user_input = String::new();
        println!("Enter what you have guesed: ");
        //using io to read what user has entered to the command line and insert it to user_input variable.
        let _stdin: usize = io::stdin()
            .read_line(&mut user_input)
            .expect("yeah some message");
        //user_input is an string variable, but we want to compare it with our secret number, so we need to convert it to an interger. line below does converting process.
        let user_input: u32 = user_input
            .trim()
            .parse()
            .expect("Please type a number dude!");
        println!("input: {} ", user_input);
        //module compare(cmp) is used. codes below is quite simple you can read it.
        match user_input.cmp(&_sys_secret_number) {
            Ordering::Equal => {
                println!("You won dude");
                break;
            }
            Ordering::Greater => println!("The secret number is smaller"),
            Ordering::Less => println!("The secret number is bigger"),
        }
    }
}
