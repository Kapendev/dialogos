use dialogos::*;

fn play(d: &mut Dialogue) {
    while !d.has_end() {
        if d.has_menu() {
            let choices = d.choices();
            // Print choices.
            println!("\nEnter a number:");
            for (i, text) in choices.iter().enumerate() {
                println!("{} => {}", i + 1, text);
            }
            // Get player choice.
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Oops error!");
                if let Ok(choice) = input.trim().parse::<usize>() {
                    if choice > 0 && choice <= choices.len() {
                        d.choose(choice - 1);
                        break;
                    }
                }
            }
            println!();
        }
        let line = d.line();
        println!("{}: {}", line.info, line.cont);
        d.next();
    }
}

fn main() {
    let gigi = |cont| text("Gigi", cont);

    let mut d = Dialogue::new(vec![
        gigi("What should I do?"),
        menu("Coffee||Tea||Sleep", "Drink coffee.||Drink tea.||Go sleep."),
        label("Coffee"),
        gigi("I drink the coffee."),
        end(),
        label("Tea"),
        gigi("I drink the tea."),
        end(),
        label("Sleep"),
        gigi("I drink the sleep."),
    ]);
    play(&mut d);
}
