use dialogos::*;

fn main() {
    let alex = |cont| text("Alex", cont);

    let mut d = Dialogue::new(vec![
        alex("Hello world."),
        alex("Something something."),
        alex("The end."),
    ]);

    while !d.has_end() {
        let line = d.line();
        println!("{}: {}", line.info, line.cont);
        d.next();
    }
}
