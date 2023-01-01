// By default, Rust has a set of items defined in the standard library that it brings into the scope of every program.
// This set is called the prelude

// To obtain user input and then print the result as output, we need to bring the io input/output library into scope.
use rand::Rng;

// bringing a type called std::cmp::Ordering into scope from the standard library.
// The Ordering type is another enum and has the variants Less, Greater, and Equal.
// These are the three outcomes that are possible when you compare two values.
use std::cmp::Ordering;

use std::io;

fn main() {
    println!("Guess the Number");

    // First we add the line use rand::Rng;.
    // The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.

    // In the first line, we call the rand::thread_rng function that gives us the particular random number generator we’re going to use:
    // one that is local to the current thread of execution and is seeded by the operating system.

    // Then we call the gen_range method on the random number generator.
    // This method is defined by the Rng trait that we brought into scope with the `use rand::Rng;` statement.
    // The gen_range method takes a range expression as an argument and generates a random number in the range.
    // The kind of range expression we’re using here takes the form start..=end
    // and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    loop {
        println!("Please input you guess");

        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        let mut guess = String::new();

        // Now we’ll call the stdin function from the io module, which will allow us to handle user input

        // If we hadn’t imported the io library with use std::io; at the beginning of the program,
        // we could still use the function by writing this function call as std::io::stdin.

        io::stdin()
            // like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.
            .read_line(&mut guess)
            // read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value.
            // Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states.
            // We call each possible state a variant.
            // Result’s variants are Ok and Err.
            // The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value.
            // The Err variant means the operation failed, and Err contains information about how or why the operation failed.
            // Values of the Result type, like values of any type, have methods defined on them.
            // An instance of Result has an expect method that you can call.
            // If this instance of Result is an Err value, expect will cause the program to crash
            // and display the message that you passed as an argument to expect.
            // If the `read_line` method returns an Err, it would likely be the result of an error coming from the underlying operating system.
            // If this instance of Result is an Ok value,
            // expect will take the return value that Ok is holding and return just that value to you so you can use it.
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // We switch from an expect call to a match expression to move from crashing on an error to handling the error.
        // Remember that parse returns a Result type and Result is an enum that has the variants Ok and Err.

        // If parse is able to successfully turn the string into a number, it will return an Ok value that contains the resultant number.
        // That Ok value will match the first arm’s pattern,
        // and the match expression will just return the num value that parse produced and put inside the Ok value.
        // That number will end up right where we want it in the new guess variable we’re creating.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // A `crate` is a collection of Rust source code files.
        // The project we’ve been building is a binary `crate`, which is an executable.
        // The `rand` crate is a library crate,
        // which contains code that is intended to be used in other programs and can’t be executed on its own.

        println!("You guessed: {guess}");

        // The cmp method compares two values and can be called on anything that can be compared.
        // It takes a reference to whatever you want to compare with: here it’s comparing guess to secret_number.
        // Then it returns a variant of the Ordering enum we brought into scope with the use statement.
        // We use a match expression to decide what to do next based on which variant of Ordering
        // was returned from the call to cmp with the values in guess and secret_number.

        // A match expression is made up of arms.
        // An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.
        // Rust takes the value given to match and looks through each arm’s pattern in turn.
        // Patterns and the match construct are powerful Rust features:
        // they let you express a variety of situations your code might encounter and they make sure you handle them all.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}

// In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.
// To make a variable mutable, we add mut before the variable name

// When we include an external dependency,
// Cargo fetches the latest versions of everything that dependency needs from the registry,
// which is a copy of data from Crates.io.
// Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

// Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project
