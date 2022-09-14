use dialogos::*;

fn main() {
    let mia = |cont| text("Mia", cont);
    let alucard = |cont| text("__name", cont);

    let mut d = Dialogue::new(vec![
        variable("name", "???"),
        mia("What's your name?"),
        alucard("They call me Alucard."),
        variable("name", "Alucard"),
        mia("__name..."),
        mia("HAHAHA!"),
        alucard("What?"),
    ]);

    while !d.has_end() {
        let line = d.line();
        println!("{}: {}", line.info, line.cont);
        d.next();
    }
}
