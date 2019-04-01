use std::io;
use game::utils;
mod game;

fn main() {
    println!("___________.__     ___________           ___________                      ");
    println!("\\__    ___/|__| ___\\__    ___/____    ___\\__    ___/___   ____         ");
    println!("  |    |   |  |/ ___\\|    |  \\__  \\ _/ ___\\|    | /  _ \\_/ __ \\     ");
    utils::sleep(300);
    println!("  |    |   |  \\  \\__ |    |   / __ \\\\  \\___ |    |(  <_> )  ___/     ");
    println!("  |____|   |__|\\___  >____|  (____  /\\___  >____| \\____/ \\___  >      ");
    println!("                   \\/             \\/     \\/                  \\/       ");
    utils::sleep(250);
    println!("   _____        .__  ___________    .___.__  __  .__                      ");
    println!("  /  _  \\_______|__| \\_   _____/  __| _/|__|/  |_|__| ____   ____       ");
    println!(" /  /_\\  \\_  __ \\  |  |    __)_  / __ | |  \\   __\\  |/  _ \\ /    \\ ");
    utils::sleep(200);
    println!("/    |    \\  | \\/  |  |        \\/ /_/ | |  ||  | |  (  <_> )   |  \\\\ ");
    println!("\\____|__  /__|  |__| /_______  /\\____ | |__||__| |__|\\____/|___|  /    ");
    utils::sleep(200);
    println!("        \\/                   \\/      \\/                         \\/    ");
    utils::sleep(200);
    println!("Welcom to Ari's TicTacToe");
    menu_main();
}

fn menu_main() {
    print!("
Please select a game mode:
    (1) Single Player
    (2) Multi Player
    (3) Human-less Player
    (4) Quit
Please choose a game type [1,2,3,4]: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut uinput = String::new();
    io::stdin().read_line(&mut uinput).expect("Failed to read line");;
    uinput = uinput.trim().to_string();
    if uinput == "1" {
        game::single_player();
        menu_main();
    }   else if uinput == "2" {
        game::multi_player();
        menu_main();
    }   else if uinput == "3" {
        game::humanless();
        menu_main();
    }   else if (uinput == "4") || (uinput.to_lowercase().starts_with("q")) {
        println!("Goodbye!");
    }   else {
        println!("Invalid input!");
        menu_main();
    }
}