use std::{thread, time, io};

pub fn print_baloons() {
    println!("__$$$$$$$$$                  ");
    println!("_$$________$$                ");
    println!("$____________$               ");
    sleep(400);
    println!("$____________$___$$$$$$$$    ");
    sleep(350);
    println!("$____________$_$$________$$  ");
    println!("$____________$$____________$ ");
    println!("_$_________$$_$____________$ ");
    sleep(300);
    println!("__$$$____$$$__$____________$ ");
    println!("_____$$$$_____$____________$ ");
    println!("_______$$______$_________$$  ");
    sleep(250);
    println!("______$$________$$$____$$    ");
    println!("_____$$____________$$$$      ");
    sleep(250);
    println!("____$________________$$      ");
    println!("____$_______________$$       ");
    println!("____$$_____________$$        ");
    println!("_____$$___________$          ");
    sleep(150);
    println!("_______$$_________$          ");
}

pub fn input(text: String) -> String {
    print!("{}", text);
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut uinput = String::new();
    io::stdin().read_line(&mut uinput).expect("Failed to read line");;
    uinput = uinput.trim().to_string();
    println!();
    return uinput;
}

pub fn clear_console() {
    print!("\r");
}

pub fn sleep(miliseconds: u64) {
    thread::sleep(time::Duration::from_millis(miliseconds));
}