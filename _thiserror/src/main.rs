/*
    When dealing with different error types in Rust, you can encounter the following situation:
    - You have a function that returns a Result<T, E>
    - The body of the function calls multiple other functions to do some work
    - Not all of these functions return the same error type

    This is a problem because Rust Result<T, E> lets you specify only one type for the error.
*/

#[derive(Debug)]
struct ErrorA;

impl Display for ErrorA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorA")
    }
}

#[derive(Debug)]
struct ErrorB;

impl Display for ErrorB {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorB")
    }
}

fn main() -> Result<(), ErrorA> {
    let _function1_ok_result = function1_that_may_fail()?;
    // let _function2_ok_result = _function2_that_may_fail()?; // <- Here's the problem
    Ok(())
}

fn function1_that_may_fail() -> Result<(), ErrorA> {
    Ok(())
}

fn _function2_that_may_fail() -> Result<(), ErrorB> {
    Ok(())
}

/*
    To get around this, you have 2 options:
        - Make the error types compatible with each other (structured)
        - Use a dynamic error type (dynamic)

    In this example, we'll focus on the first option, the structured approach.

    You'll have to implement the From trait for each error type you want to convert to the main error type.
    And if you're dealing with error types of external crates, you'd have to apply the Newtype pattern and reverse the conversion.
*/

impl From<ErrorB> for ErrorA {
    fn from(_: ErrorB) -> Self {
        ErrorA
    }
}

impl From<ErrorA> for ErrorB {
    fn from(_: ErrorA) -> Self {
        ErrorB
    }
}

// Now this works, as ErrorB can be treated as an ErrorA
fn _better_main() -> Result<(), ErrorA> {
    let _function1_ok_result = function1_that_may_fail()?;
    let _function2_ok_result = _function2_that_may_fail()?;
    Ok(())
}

/*
    As this is tedious to do manually, especially when converting data,
    the Rust community has created the thiserror crate to help you with this.

    It provides a macro that generates the From implementations for you.
    thiserror also generates the Display implementation for you.

    To add thiserror to your project, run the following command:
    cargo add thiserror
*/

use std::fmt::{self, Display, Formatter};

use thiserror::Error;

#[derive(Error, Debug)] // Derive the Error trait from thiserror
#[error("ErrorA")] // thiserror macro to implement the Display trait
struct BetterErrorA {
    #[from] // thiserror macro to implement the From trait
    source: BetterErrorB,
}

#[derive(Error, Debug)]
#[error("ErrorB")]
struct BetterErrorB;

// And now we can also use BetterErrorA as the error type for our main function, as BetterErrorB can be treated as BetterErrorA
fn _even_better_main() -> Result<(), BetterErrorA> {
    let _function1_ok_result = _better_function1_that_may_fail()?;
    let _function2_ok_result = _better_function2_that_may_fail()?;
    Ok(())
}

fn _better_function1_that_may_fail() -> Result<(), BetterErrorA> {
    Ok(())
}

fn _better_function2_that_may_fail() -> Result<(), BetterErrorB> {
    Ok(())
}

/*
    While dealing with multiple error types in Rust can introduce some boilerplate code,
    the thiserror crate can help you reduce it.
    Defining your own error types has the advantage of not losing any information about the error.
    When calling your function, you can match on the error type to handle each error differently.
    This is not possible when using the dynamic approach.
    Also, as the error types are known at compile time, the compiler can resolve everything at compile time,
    while the dynamic approach introduces a runtime cost.

    For the dynamic approach, check out the _anyhow directory.
*/
