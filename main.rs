fn main()
{
}

fn rules()
{
	writeln!("-- RULES --");
	writeln!("You play as the better, placing bets against the roll of the caster.");
}

fn help()
{
	writeln!("-- HELP --");
	writeln!("h - brings up the help screen.");
	writeln!("r - brings up the rules screen.");
	writeln!("a - brings up the about screen.");
	writeln!("q - ends the program.\n");	
}

fn welcome()
{
	println!("-- WELCOME TO HAZARD --");
	println!("Bet at your own risk...\n");
}

fn about()
{
	writeln!("-- ABOUT --");
	writeln!("Hazard is a very old English dice game that has been around since the\n13th century and is a predeccessor to Crabs.");
	writeln!("Geoffrey Chaucer even refers to the game in the Canterbury Tales!");
	writeln!("Despite its complicated rules, hazard was very popular in the 17th\nand 18th centuries and was often played for money.");
	writeln!("Hazard is generally a game played between two people - a caster and a better. However, hazard also supports multiple betters.");
	writeln!("This implementation only supports one person - a better playing against the computer caster.");
	writeln!("\nCreators:");
	writeln!("Andrew Young: andrew.young@oit.edu");
	writeln!("Logan Francisco: logan.francisco@oit.edu");
	writeln!("Rowan Parker: rowan.parker@oit.edu\n");
}
