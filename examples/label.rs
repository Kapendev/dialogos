use dialogos::*;

fn main() {
    let ferris = |cont| text("Ferris", cont);

    let mut d = Dialogue::new(vec![
        label("The Beginning"),
        ferris("Hello world."),
        jump("The End"),
        ferris("Something something."),
        ferris("The end."),
        label("The End"),
    ]);

    while !d.has_end() {
        let line = d.line();
        println!("{}: {}", line.info, line.cont);
        d.next();
    }
}
