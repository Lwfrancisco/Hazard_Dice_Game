use std::io;

struct Player {
    winnings: i32,
    bet: i32,
}

fn main()
{
    /*
    Note: Variables are immutable unless initialized as "mut".
    Variables do not necessarily need to be assigned a type, as the Rust compiler will attempt to figure it out for you.
    Data Types:
        u8 = unsigned 8-bit integer
        i8 = signed 8-bit integer
        etc.
    Tables are available for float, char, etc distinctions online. I found this useful: https://www.tutorialspoint.com/rust/rust_data_types.htm
    */

    // Number of players.
    let player_count:u8 = 2;

    // Terminated is set when player quits the game or the game is won.
    let mut terminated:bool = false;

	let mut player = Player{ 0, 0 }; 
	
    let mut user_input = String::new();

    welcome();
    help();

    print_unicode_die(1);
    print_unicode_die(2);
    print_unicode_die(3);
    print_unicode_die(4);
    print_unicode_die(5);
    print_unicode_die(6);
	
    while !terminated
    {
        // Begin a "round" of gameplay.
       for player_turn in 0..player_count 
        {
            println!("Player {}'s turn.", player_turn);

            // Obtain input and evaluate it
            io::stdin().read_line(&mut user_input)
                .expect("Failed to read line");

            //let _input = user_input.as_str();

            // truncate all but the first character.
            user_input.truncate(1);
            
            if user_input == "h"
            {
                help();
			}
            else if user_input == "r"
            {
                rules();
            }
            else if user_input == "a"
            {
                about();
            }
			else if user_input == "b"
			{
				place_bet(player);
			}
            else if user_input == "q"
            {
                terminated = true;
                break;
            }
        }

        // Check if game is won
        if terminated == false
        {
            terminated = win_condition();
        }
    }
}

/*********************************************************************
 * method name: rules
 * purpose: display the rules of the game
 * parameters: none
 * return: none
 * ******************************************************************/ 
fn rules()
{
    println!("\t\t-- RULES --");
    println!("You play as the better, placing bets against the roll of the caster.");
    println!("Before each roll, you will place a bet on whether the caster will lose, win, or throw a chance.");
    println!("In each round, the caster chooses a number between 5 and 9. This is called the main.");
    println!("Then, the caster rolls two dice.");
    println!("If the caster throws the main, he automatically wins and you lose.");
    println!("If the caster throws a 2 or 3, he automatically loses and you win.");
    println!("If the caster throws a 11 or 12, the results depend on the main: ");
    println!("\t-> if the main is 5 or 9, the caster loses.");
    println!("\t-> if the main is 6 or 8, the caster loses with a roll of 11 but wins with a roll of 12.");
    println!("\t-> if the main is 7, the caster wins with a roll of 12 but loses with a roll of 11.");
    println!("If the caster does not throw the main, 2, 3, 11, or 12, then the roll is called the chance.");
    println!("The caster gets to roll again:");
    println!("\t-> if the caster throws the chance, he wins and you lose.");
    println!("\t-> if the caster throws the main, he loses and you win.");
    println!("\t-> if the caster throws neither, he keeps rolling until he either throws the chance or main.");
}

/*********************************************************************
 * method name: help
 * purpose: display the options of the game
 * parameters: none
 * return: none
 * ******************************************************************/ 
fn help()
{
	println!("\t\t-- HELP --");
	println!("h - brings up the help screen.");
	println!("r - brings up the rules screen.");
	println!("a - brings up the about screen.");
	println!("q - ends the program.\n");	
}

/*********************************************************************
 * method name: welcome
 * purpose: display welcome message
 * parameters: none
 * return: none
 * ******************************************************************/ 
fn welcome()
{
	println!("\t\t-- WELCOME TO HAZARD --");
	println!("\n\tBet at your own risk...\n");
}

/*********************************************************************
 * method name: about
 * purpose: display the history and creator information of the game
 * parameters: none
 * return: none
 * ******************************************************************/ 
fn about()
{
	println!("\t\t-- ABOUT --");
	println!("Hazard is a very old English dice game that has been around since the\n13th century and is a predeccessor to Crabs.");
	println!("Geoffrey Chaucer even refers to the game in the Canterbury Tales!");
	println!("Despite its complicated rules, hazard was very popular in the 17th\nand 18th centuries and was often played for money.");
	println!("Hazard is generally a game played between two people - a caster and a better. However, hazard also supports multiple betters.");
	println!("This implementation only supports one person - a better playing against the computer caster.");
	println!("\nCreators:");
	println!("\tAndrew Young: andrew.young@oit.edu");
	println!("\tLogan Francisco: logan.francisco@oit.edu");
	println!("\tRowan Parker: rowan.parker@oit.edu\n");
}

/*********************************************************************
 * method name: print_ascii_die
 * purpose: display an ASCII image of a single die depending on what
 *  what has been rolled.
 * parameters: unsigned 8-bit integer for number rolled
 * return: none
 * ******************************************************************/ 
fn print_ascii_die(d1: u8)
{
    println!(" ---");
    //  ---
    // |   |
    // | o |
    // |   |
    //  ---
    if d1 == 1
    {
        println!("|   |");
        println!("| o |");
        println!("|   |");
    }
    //  ---
    // |o  |
    // |   |
    // |  o|
    //  ---
    else if d1 == 2
    {
        println!("|o  |");
        println!("|   |");
        println!("|  o|");
    }
    //  ---
    // |o  |
    // | o |
    // |  o|
    //  ---
    else if d1 == 3
    {
        println!("|o  |");
        println!("| o |");
        println!("|  o|");
    }
    //  ---
    // |o o|
    // |   |
    // |o o|
    //  ---
    else if d1 == 4
    {
        println!("|o o|");
        println!("|   |");
        println!("|o o|");
    }
    //  ---
    // |o o|
    // | o |
    // |o o|
    //  ---
    else if d1 == 5
    {
        println!("|o o|");
        println!("| o |");
        println!("|o o|");
    }
    //  ---
    // |o o|
    // |o o|
    // |o o|
    //  ---
    else 
    {
        println!("|o o|");
        println!("|o o|");
        println!("|o o|");
    }
    println!(" ---\n");
}

/*********************************************************************
 * method name: print_unicode_die
 * purpose: display an Unicode image of a single die depending on what
 *  what has been rolled.
 * parameters: unsigned 8-bit integer for number rolled
 * return: none
 * ******************************************************************/ 
fn print_unicode_die(d1:u8)
{
    println!(" ―――――");
    //  ―――――
    // ｜   ｜
    // ｜ • ｜
    // ｜   ｜
    //  ―――――
    if d1 == 1
    {
        println!("｜   ｜");
        println!("｜ • ｜");
        println!("｜   ｜");
    }
    //  ―――――
    // ｜•  ｜
    // ｜   ｜
    // ｜  •｜
    //  ―――――
    else if d1 == 2
    {
        println!("｜•  ｜");
        println!("｜   ｜");
        println!("｜  •｜");
    }
    //  ―――――
    // ｜•  ｜
    // ｜ • ｜
    // ｜  •｜
    //  ―――――
    else if d1 == 3
    {
        println!("｜•  ｜");
        println!("｜ • ｜");
        println!("｜  •｜");
    }
    //  ―――――
    // ｜• •｜
    // ｜   ｜
    // ｜• •｜
    //  ―――――
    else if d1 == 4
    {
        println!("｜• •｜");
        println!("｜   ｜");
        println!("｜• •｜");
    }
    //  ―――――
    // ｜• •｜
    // ｜ • ｜
    // ｜• •｜
    //  ―――――
    else if d1 == 5
    {
        println!("｜• •｜");
        println!("｜ • ｜");
        println!("｜• •｜");
    }
    //  ―――――
    // ｜• •｜
    // ｜• •｜
    // ｜• •｜
    //  ―――――
    else
    {
        println!("｜• •｜");
        println!("｜• •｜");
        println!("｜• •｜")
    }
    println!(" ―――――\n");
}

/*********************************************************************
 * method name: print_emoji_die
 * purpose: display an Unicode emoji of a single die depending on what
 *  what has been rolled.
 * parameters: unsigned 8-bit integer for number rolled
 * return: none
 * ******************************************************************/ 
fn print_emoji_die(d1:u8)
{  
    let face:char =
    // die face-1
    if d1 == 1
    { '⚀' }
    // die face-2
    else if d1 == 2
    { '⚁' }
    // die face-3
    else if d1 == 3
    { '⚂' }
    // die face-4
    else if d1 == 4
    { '⚃' }
    // die face-5
    else if d1 == 5
    { '⚄' }
    // die face-6
    else 
    { '⚅' };

    println!("{}\n", face);
}

/*********************************************************************
 * method name: place_bet
 * purpose: determine wether the player wants to make a bet
 * return: player's guess of outcome
 * ******************************************************************/ 
fn place_bet(player:Player) -> bool
{
    let mut cont:bool = false;
    let mut bet:bool = false;
    let mut outcome:bool = false;
    let mut bet_amount: i32 = 0;

    while cont == false {
        println!("Would you like to bet?");

        let mut decision = String::new();
        io::stdin()
            .read_line(&mut decision)
            .expect("failed to read from stdin");
    
        if decision.trim().to_ascii_lowercase() == "yes"{
            println!("You want to place a bet");
            cont = true;
            bet = true;
        } else if decision.trim().to_ascii_lowercase() == "no"{
            println!("You do not want to place a bet");
            cont = true;
        } else {
            println!("Please give a clearer answer");
        }
    }

    cont = false;

    if bet == true {
        while cont == false {
            println!("How much would you like to bet?");

            let mut decision = String::new();
            io::stdin()
                .read_line(&mut decision)
                .expect("failed to read from stdin");

            let trimmed = decision.trim();
            match trimmed.parse::<i32>() {
                Ok(i) => {
                        println!("Your bet: ${}", i);
                        player.bet = i;
                        cont = true
                }
                Err(..) => println!("That was not a proper bet"),
            };

        }
	    
	cont = false;
	
	while cont == false {
		println!("Would you like to bet for a win?");

            let mut decision = String::new();
            io::stdin()
               .read_line(&mut decision)
               .expect("failed to read from stdin");
    
            if decision.trim().to_ascii_lowercase() == "yes"{
               println!("You want the caster to win");
               cont = true;
	       outcome = true;
            }  else if decision.trim().to_ascii_lowercase() == "no"{
               println!("You want the caster to lose");
               cont = true;
            } else {
               println!("Please give a clearer answer");
            }
	
	}
    }
	
    return outcome;
}

/**
 * win_condition -> bool
 * Determines if the caster has "won"
 * 
 * Returns true as a placeholder.
 */
fn win_condition() -> bool
{
    return false;
}
