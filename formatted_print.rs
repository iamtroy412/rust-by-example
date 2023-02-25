use std::fmt;

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character after a
    // `:`.
    println!("Base 10:              {}", 69420);
    println!("Base 2 (binary):      {:b}", 69420);
    println!("Base 8 (octal):       {:o}", 69420);
    println!("Base 16 (hex):        {:x}", 69420);
    println!("Base 16 (HEX):        {:X}", 69420);

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    //and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1);

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0<width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default
    struct Blah(i32);

    impl fmt::Display for Blah {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // This will not compile because `Structure` does not implement
    // fmt::Display
    println!("This struct `{}` won't print...", Blah(42));
    // ...now it does because I implemented fmt::Display for Blah

    // Control the number of decimals printed. ie precision
    println!("PI is roughly {:.3}", 3.141592);
}
