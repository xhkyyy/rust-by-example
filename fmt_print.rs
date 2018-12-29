fn main() {
    println!("{} days", 31);

    println!("{} {} days", "->", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // you can right-align text with a specified width.
    println!("{number:width$}", number = 1, width = 6);

    // you can pad numbers with extra zeroes.
    println!("{number:>0width$}", number = 1, width = 6);

    /*
    Wit will even check to make sure the correct number of arguments are used.
    println!("My name is {0}, {1}, {0}", "Bond");
    */

    // Create a structre wich contains on `i32`. Name it `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    /*
    However, custom types such as this structure require more complicated handling. This will not work.
    println!("This struct `{}` won't print...", Structure(3));
     */

    println!("{:?} {:?}", object = "the lazy dog", verb = 10000);

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
