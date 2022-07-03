// hello there!
mod colours;
use std::{thread, time::Duration};
use enigo::*;


fn main() {
    // green tea ascii art
    let green_tea;

    green_tea = format!(
        "
        {green}
  ▄▄▄                                    ▄▄▄▄▄▄▄              
▄▀   ▀  ▄ ▄▄   ▄▄▄    ▄▄▄   ▄ ▄▄            █     ▄▄▄    ▄▄▄  
█   ▄▄  █▀  ▀ █▀  █  █▀  █  █▀  █           █    █▀  █  ▀   █ 
█    █  █     █▀▀▀▀  █▀▀▀▀  █   █   ▀▀▀     █    █▀▀▀▀  ▄▀▀▀█ 
 ▀▄▄▄▀  █     ▀█▄▄▀  ▀█▄▄▀  █   █           █    ▀█▄▄▀  ▀▄▄▀█

          __________________
         /                  \\
        |                    |
        |\\__________________/|
        |                    |-----.
        |         /\\         |-----. \\
        |        /  \\        |      \\ \\
        |       / / |        |       | |
        |       | | |        |       | |
        |       \\ | /        |      / /
        |          \\\\        |_____/ /
        |           \\\\       |______/
        |____________________|
        {reset}
        ",
        green = colours::green,
        reset = colours::reset
    );

    print!("{}\n", green_tea);

    // ask user for duration
    let mut interval = String::new();
    println!("Enter interval in minutes: ");
    std::io::stdin().read_line(&mut interval).expect("Failed to read line");
    // magic loop
    loop{
        let mut enigo = Enigo::new();
        enigo.key_click(Key::Shift);
        print!("Pressed shift to keep the screen alive\n");
        thread::sleep(Duration::from_secs_f64(interval.trim().parse::<f64>().unwrap() * 60.0));
    }
}
