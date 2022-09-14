use dialogos::*;

mod info {
    pub const NAME: usize = 0;
    pub const EMOTION: usize = 1;
}

mod emotion {
    pub const DEFAULT: &str = "Default";
    pub const HAPPY: &str = "Happy";
}

mod face {
    pub const DEFAULT: &str = "o-o";
    pub const HAPPY: &str = "^-^";
}

fn main() {
    let emma_d = |cont| text("Emma||Default", cont);
    let emma_h = |cont| text("Emma||Happy", cont);

    let mut d = Dialogue::new(vec![
        emma_d("..."),
        emma_d("It's a nice day today."),
        emma_h("..."),
    ]);

    while !d.has_end() {
        let line = d.line();
        let data = split(&line.info);

        let n = &data[info::NAME];
        let f = match data[info::EMOTION].as_str() {
            emotion::DEFAULT => face::DEFAULT,
            emotion::HAPPY => face::HAPPY,
            _ => "",
        };

        println!("{} ({}): {}", n, f, line.cont);
        d.next();
    }
}
