use dialogos::*;

fn main() {
    let momo = |cont| text("Momo", cont);

    let mut d = Dialogue::new(vec![
        momo("Hello world."),
        momo("Something something."),
        end(),
        momo("The end."),
    ]);

    while !d.has_end() {
        let line = d.line();
        println!("{}: {}", line.info, line.cont);
        d.next();
    }
}
