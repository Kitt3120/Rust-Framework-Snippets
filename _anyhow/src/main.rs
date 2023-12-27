/*
    When dealing with different error types in Rust, you can encounter the following situation:
    - You have a function that returns a Result<T, E>
    - The body of the function calls multiple other functions to do some work
    - Not all of these functions return the same error type

    This is a problem because Rust Result<T, E> lets you specify only one type for the error.
*/

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

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

    In this example, we'll focus on the second option, the dynamic approach.

    The dynamic approach is to use a trait object as the error type.
    This allows you to return any error type that implements the trait.
*/

impl Error for ErrorA {} // Make sure our custom error type is marked as an error
impl Error for ErrorB {} // Make sure our custom error type is marked as an error

fn _better_main() -> Result<(), Box<dyn Error>> {
    let _function1_ok_result = function1_that_may_fail()?;
    let _function2_ok_result = _function2_that_may_fail()?;
    Ok(())
}

/*
    This can be further simplified by using the anyhow crate.

    To add the anyhow crate to your project, run the following command:
    cargo add anyhow

    Then, you can use the anyhow::Error type as your error type.
*/

use anyhow::Result;

// Notice that we only have to specify the valid return type
fn _even_better_main() -> Result<()> {
    let _function1_ok_result = function1_that_may_fail()?;
    let _function2_ok_result = _function2_that_may_fail()?;
    Ok(())
}

/*
    While using this dynamic approach or even anyhow makes things easier, it's not always the best solution.
    The dynamic approach is less efficient than the structured approach because it uses dynamic dispatch,
    which introduces a runtime cost.
    Also, when calling our function, we can only match on the trait object, not the concrete error type.
    This means we can't handle different errors differently.
    This makes it harder to use our function in a way that's idiomatic to Rust.
    So when developing libraries, it's best to use the structured approach.
    However, the dynamic approach is fine as long as you're fine with it and are aware of the tradeoffs.

    For the structured approach, check out the _thiserror directory.
*/
