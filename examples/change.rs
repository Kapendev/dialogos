use dialogos::*;

fn play(d: &mut Dialogue) {
    while !d.has_end() {
        let line = d.line();
        println!("{}: {}", line.info, line.cont);
        d.next();
    }
}

fn main() {
    let nicoolo = |cont| text("Nicoolo", cont);
    let babywog = |cont| text("Baby WOGUE", cont);

    let mut d = Dialogue::new(vec![nicoolo("Hello."), nicoolo("I like KDE.")]);
    play(&mut d);

    d.change(vec![babywog("Hello."), babywog("I like GTK and Rust.")]);
    play(&mut d)
}
