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
    
    // Player turn. 0 is the caster and 1+ are the players.
    let mut player_turn:u8 = 0;

    // Terminated is set when player quits the game or the game is won.
    let mut terminated:bool = false;

    welcome();
    help();

    while !terminated
    {
        // Begin a "round" of gameplay.
        for player_turn in 0..player_count 
        {
            println!("Player {}'s turn.", player_turn);
        }

        // Check if game is won
        terminated = win_condition();
    }

}

fn rules()
{
	println!("-- RULES --");
	println!("You play as the better, placing bets against the roll of the caster.");
}

fn help()
{
	println!("-- HELP --");
	println!("h - brings up the help screen.");
	println!("r - brings up the rules screen.");
	println!("a - brings up the about screen.");
	println!("q - ends the program.\n");	
}

fn welcome()
{
	println!("-- WELCOME TO HAZARD --");
	println!("Bet at your own risk...\n");
}

fn about()
{
	println!("-- ABOUT --");
	println!("Hazard is a very old English dice game that has been around since the\n13th century and is a predeccessor to Crabs.");
	println!("Geoffrey Chaucer even refers to the game in the Canterbury Tales!");
	println!("Despite its complicated rules, hazard was very popular in the 17th\nand 18th centuries and was often played for money.");
	println!("Hazard is generally a game played between two people - a caster and a better. However, hazard also supports multiple betters.");
	println!("This implementation only supports one person - a better playing against the computer caster.");
	println!("\nCreators:");
	println!("Andrew Young: andrew.young@oit.edu");
	println!("Logan Francisco: logan.francisco@oit.edu");
	println!("Rowan Parker: rowan.parker@oit.edu\n");
}

/**
 * win_condition -> bool
 * Determines if the caster has "won"
 * 
 * Returns true as a placeholder.
 */
fn win_condition() -> bool
{
    return true;
}