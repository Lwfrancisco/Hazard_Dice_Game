use std::io;
use rand::Rng;

struct Player {
    winnings: i32,
    bet: i32,
	outcome_guess: bool,
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
    let player_count:u8 = 1;

    // Which player's turn is it? 0 is caster.
    let mut player_turn:u8 = 0;

    // Terminated is set when player quits the game or the game is won.
    let mut terminated:bool = false;

	let mut player = Player{ winnings:0, bet:0, outcome_guess:false }; 
	
    let mut user_input = String::new();

    // Number of throws for caster
    let mut caster_throws = 0;

    // What the caster has rolled
    let mut caster_roll = 0;

    // Variable will store the caster's main roll.
    let mut caster_main:u8 = 0;

    welcome();
    help();
	
    while !terminated
    {
        // Begin a "round" of gameplay.

        println!("Player {}'s turn.", player_turn + 1);

        if caster_main == 0
        {
            caster_main = roll_caster_main();
            println!("The caster's main is: {}\n", caster_main);
        }

        // if user_input is assigned to some value
        if !(user_input == "") 
        {
            user_input = "".to_string();
        }

        // Obtain input and evaluate it
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");

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
            place_bet(&mut player, caster_main);
            player_turn = player_turn + 1;
        }
        else if user_input == "q"
        {
            terminated = true;
        }

        // if we have cycled through all players.
        if player_turn >= player_count
        {
            player_turn = 0;
        }

         // Check if game is won
        if terminated == false && user_input == "b"
        {
            let mut keep_rolling = true;

            while (keep_rolling)
            {
                caster_roll = roll();

                let round_result = win_condition(caster_main, caster_roll);

                if (round_result == 0) {
                    println!("Caster has lost!");
                    money_distribution(false, &mut player);
					println!("Player has won ${}, player.winnings);
					terminated = truw;
                    keep_rolling = false;
                }
                else if round_result == 2
                {
                    println!("Caster has won!");
                    money_distribution(true, &mut player);
					println!("Player has won ${}, player.winnings);
					terminated = true;
                    keep_rolling = false;

                }
                else if round_result == 1
                {
                    println!("Caster has thrown a chance!");
                    keep_rolling = true;
                }
            }
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
    println!("b - allows the player to submit a bet.");
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
 * parameters: A reference to a Player struct to allow value changes
 * return: none
 * ******************************************************************/ 
fn place_bet(mut play:&mut Player, main:u8)
{
    let mut cont:bool = false;
    let mut bet:bool = false;

    while cont == false {
        println!("The main is {}. Would you like to bet? (yes/no)", main);

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
                        play.bet = i;
                        cont = true
                }
                Err(..) => println!("That was not a proper bet"),
            };

        }
	    
		cont = false;
	
		while cont == false {
			println!("Would you like to bet for a win? (yes/no)");

            let mut decision = String::new();
            io::stdin()
               .read_line(&mut decision)
               .expect("failed to read from stdin");
    
            if decision.trim().to_ascii_lowercase() == "yes"{
               println!("You want the caster to win");
               cont = true;
	       	   play.outcome_guess = true;
            }  else if decision.trim().to_ascii_lowercase() == "no"{
               println!("You want the caster to lose");
               cont = true;
            } else {
               println!("Please give a clearer answer");
            }
		}
    }
}

/**
 * roll_caster_main -> i8
 * Rolls until caster gets a valid main.
 * 
 * Returns value of rolled main.
 */
fn roll_caster_main() -> u8
{
    let mut roll:u8 = 0;

    while roll < 5 || roll > 9
    {
        let dice1 = rand::thread_rng().gen_range(1, 6);
        let dice2 = rand::thread_rng().gen_range(1, 6);

        roll = dice1 + dice2;

        if roll >= 5 && roll <= 9
        {
            print_unicode_die(dice1);
            print_unicode_die(dice2);
        }
    }

    return roll;
}

/**
 * win_condition -> bool
 * Determines if the caster has "won", "lost", or "chanced"
 * 
 * Returns 0 if lost, 1 if chance, or 2 if won.
 */
fn win_condition(_main:u8, _roll:u8) -> u8
{

    // Check if caster has lost
    if _roll == 2 || _roll == 3
    {
        return 0; // caster will always lose on these
    }
    else if (_main == 5 || _main == 9) && (_roll == 11 || _roll == 12)
    {
        return 0;
    }
    else if (_roll == 1) && (_main == 6 || _main == 8)
    {
        return 0;
    }
    else if _roll == 12 && _main == 7
    {
        return 0;
    }

    // Check if caster has won
    if _main == 5 && _main == _roll
    {
        return 2;
    }
    else if _main == 6 && (_main == _roll || _roll == 12)
    {
        return 2;
    }
    else if _main == 7 && (_main == _roll || _roll == 11)
    {
        return 2;
    }
    else if _main == 9 && _main == _roll
    {
        return 2;
    }
    else if _main == 8 && (_main == _roll || _roll == 12)
    {
        return 2;
    }

    // Else all, roll is a chance
    return 1;

}

/**
 * money_distribution
 * Tallies out the money
 */
fn money_distribution(game_outcome:bool, mut play:&mut Player)
{
    if(game_outcome == play.outcome_guess)
    {
        play.winnings = play.winnings + play.bet;
    }
    else
    {
        play.winnings = play.winnings - play.bet;
    }
}


fn roll() -> u8
{
    let dice1 = rand::thread_rng().gen_range(1, 6);
    let dice2 = rand::thread_rng().gen_range(1, 6);

    print_unicode_die(dice1);
    print_unicode_die(dice2);

    return dice1 + dice2;
}
