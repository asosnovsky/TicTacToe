use std::{thread, time, io};

pub fn print_baloons() {
    println!("__$$$$$$$$$                  ");
    sleep(400);
    println!("_$$________$$                ");
    sleep(400);
    println!("$____________$               ");
    sleep(400);
    println!("$____________$___$$$$$$$$    ");
    sleep(350);
    println!("$____________$_$$________$$  ");
    sleep(350);
    println!("$____________$$____________$ ");
    sleep(350);
    println!("_$_________$$_$____________$ ");
    sleep(300);
    println!("__$$$____$$$__$____________$ ");
    sleep(300);
    println!("_____$$$$_____$____________$ ");
    sleep(300);
    println!("_______$$______$_________$$  ");
    sleep(250);
    println!("______$$________$$$____$$    ");
    sleep(250);
    println!("_____$$____________$$$$      ");
    sleep(250);
    println!("____$________________$$      ");
    sleep(200);
    println!("____$_______________$$       ");
    sleep(200);
    println!("____$$_____________$$        ");
    sleep(200);
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
    // io::stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap()
}

pub fn sleep(miliseconds: u64) {
    thread::sleep(time::Duration::from_millis(miliseconds));
}