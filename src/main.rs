use rand::Rng;

fn main() {
    let game_values :GameValues= game_init();
    let secret_number: u32 = rand::thread_rng().gen_range(game_values.lower_bound..=game_values.upper_bound);    
    let mut guess_count: u32 = 0;

    println!("Enter your number: ");

    loop {
        if guess_count == game_values.max_guess {
            println!("The secret number was: {secret_number}");
            break;
        }

        let user_input:u32 = match prompt("").trim().parse() {
            Ok(num)=>num,
            Err(_) => continue
        };
            
        println!("Your number is: {user_input}");

        if win_checker(secret_number, user_input) {
            break;
        }
        
        guess_count += 1;
        println!("{} guess(es) left", game_values.max_guess - guess_count );
    }


}


fn win_checker(secret_number:u32, user_number:u32) -> bool {
    match user_number.cmp(&secret_number) {
        std::cmp::Ordering::Greater => {
            println!("Too big");
            return false;
        },
        std::cmp::Ordering::Less => {
            println!("Too small");
            return false; 
        },
        std::cmp::Ordering::Equal => {
            println!("You win");
            return true;
        },
    };
}

fn game_init () -> GameValues {
    clear();
    let starting_value:u32 = match prompt("What is your starting number").trim().parse() {
        Ok(num) => num,
        Err(_)=> panic!("Invalid starting value"),
    };
    
    clear();
    let ending_value:u32 = match prompt("What is your ending number").trim().parse() {
        Ok(num) => num,
        Err(_)=> panic!("Invalid ending value"),
    };
    
    clear();
    let guess_value:u32 = match prompt("How many times do you want to try").trim().parse(){
        Ok(num) => num,
        Err(_)=> panic!("Invalid value for max play time"),
    };

    return GameValues::new(guess_value, starting_value, ending_value)
}

fn prompt(msg:&str) -> String {
    println!("{msg}");

    let mut input:String = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not read input");

    return input;
}

fn clear(){
    print!("\x1B[2J\x1B[1;1H");
}

struct GameValues {
    max_guess: u32,
    lower_bound: u32,
    upper_bound: u32,
}

impl GameValues {
    fn new(max_guess:u32, lower_bound:u32, upper_bound:u32) -> Self {
        return GameValues { max_guess, lower_bound, upper_bound };
    }
}