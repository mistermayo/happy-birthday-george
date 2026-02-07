use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::{io::stdout, thread, time::Duration};

fn main() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();

    println!("Happy Birthday, George.");
    println!("You awesome cat dad.\n");

    let cats = [
        "=^.^=",
        "(=^･ω･^=)",
        "(=ↀωↀ=)",
        "(=｀ω´=)",
    ];

    let width = 40;

    for step in 0..width {
        execute!(stdout, Clear(ClearType::All)).unwrap();

        println!("Happy Birthday, George.");
        println!("You awesome cat dad.\n");

        for (i, cat) in cats.iter().enumerate() {
            let indent = (step + i * 6) % width;
            println!("{:indent$}{}", "", cat, indent = indent);
        }

        thread::sleep(Duration::from_millis(120));
    }
}

