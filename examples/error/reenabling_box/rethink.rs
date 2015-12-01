use std::error;
use std::fmt;
use std::num::ParseIntError;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", match *self {
            DoubleError::EmptyVec => 
                "please use a vector with at least one element".to_owned(),
            DoubleError::Parse(ref e) => e.to_string(),
        })
    }
}

impl error::Error for DoubleError {
    fn description(&self) -> &str {
        match *self {
            // Trying to use `Display` here results in the string being immediately
            // dropped which won't compile so we have to write out the phrase again.
            DoubleError::EmptyVec => "please use a vector with at least one element",
            // This already impls `Error`, so defer to its implementation.
            DoubleError::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // No underlying cause so return `None`.
            DoubleError::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
