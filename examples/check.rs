use dialogos::*;

fn main() {
    let einstein = |cont| text("Einstein", cont);

    let mut d = Dialogue::new(vec![
        variable("val", "1 + 2"),
        einstein("1 + 2 = __val"),
        einstein("And..."),
        variable("val", "__val * __val"),
        einstein("3 * 3 = __val"),
        // Check if the math is bad.
        check("__val == 9"),
        einstein("I'm very good at math!"),
        check("__val != 9"),
        einstein("I did something wrong?"),
    ]);

    while !d.has_end() {
        let line = d.line();
        println!("{}: {}", line.info, line.cont);
        d.next();
    }
}
