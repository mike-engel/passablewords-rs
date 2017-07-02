//! `passablewords` is a password validation library which checks a password against a million of
//! the most common as well as it's ability to be cracked.
//!
//! If you're asking why use `passablewords` over `zxcvbn`, it's because `passablewords` checks
//! a password against 1,000,000 of the most common passwords. `zxcvbn` only checks 30,000.
//! `zxcvbn` is a great tool, however, and `passablewords` uses it to check the entropy of a given
//! password to make sure it's random enough on top of being unique enough. If you are ok with the
//! top 30,000 most common passwords, then you should probably use `zxcvbn`. If you want a little
//! extra, consider `passablewords`.
//!
//! While you're free to use any of the public methods, using the `check_password` function is
//! recommended since that checks for length, uniqueness, and entropy all within a single call.
//!
//! It's also important to note that this is provided as-is and doesn't prevent an attacker from
//! gaining access to, decrypting, or guessing your user's passwords. It just makes it a little
//! harder.
//!
//! # Example
//!
//! ```
//! extern crate passablewords;
//!
//! use passablewords::{check_password, PassablewordResult};
//!
//! fn main() {
//!     match check_password(password) {
//!         Ok() => println!("That password is probably pretty good!")
//!         Err(err) => match(err) {
//!             PassablewordResult::TooShort => println!("Your password should be longer than 8 characters"),
//!             PassablewordResult::TooCommon => println!("Your should be more unique"),
//!             PassablewordResult::TooSimple => println!("Your should be more random"),
//!             PassablewordResult::InternalError => println!
//!         }
//!     }
//! }
//! ```

#[macro_use]
extern crate lazy_static;
extern crate zxcvbn;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use zxcvbn::{zxcvbn, ZxcvbnError};

lazy_static! {
    static ref FILE_CONTENTS: String = {
        let mut f = File::open("src/common-passwords.txt")
            .expect("There was a problem opening the list of passwords");
        let mut file_contents = String::new();

        f.read_to_string(&mut file_contents)
            .expect("There was a problem reading the common passwords file");

        file_contents
    };
    static ref PASSWORDS: HashSet<&'static str> = {
        HashSet::from_iter(FILE_CONTENTS.lines())
    };
}

/// The suite of possible errors returned from passablewords. These represent the three checks made
/// for length, uniqueness, and entropy. If something goes wrong during the request, an
/// `InternalError` error is returned.
#[derive(Debug, PartialEq)]
pub enum PasswordError {
    /// The password is less than 8 characters and is therefore too short.
    TooShort,
    /// The password is within the list of 1,000,000 most common passwords and should not be used.
    TooCommon,
    /// The entropy of the password is too low, which means it could be easily guessable/crackable.
    /// A more random password should be used instead.
    TooSimple,
    /// The password isn't using ascii characters, a requirement that zxcvbn has
    NonAsciiPassword,
    /// Something went wrong during the password checks and a normal error couldn't be returned.
    InternalError,
}

/// The result type that will be returned from all public functions. It's simply a `Result` type
/// that either returns `Ok` or a `PasswordError`.
pub type PassablewordResult = Result<(), PasswordError>;

/// Check a password to make sure it's at least 8 characters long. While this shouldn't be used as
/// the only password check, it's a good baseline to start from.
///
/// # Example (using rocket.rs)
///
/// ```
/// match check_length(password) {
///     Ok() => status::Ok
///     Err(err) => match(err) {
///         PassablewordResult::TooShort => status::BadRequest("Your password should be longer than 8 characters")
///     }
/// }
/// ```
pub fn check_length(password: &str) -> PassablewordResult {
    if password.len() >= 8 {
        Ok(())
    } else {
        Err(PasswordError::TooShort)
    }
}

/// Check a password to make sure it's not within the top million most common passwords.
///
/// # Example (using rocket.rs)
///
/// ```
/// match check_uniqueness(password) {
///     Ok() => status::Ok
///     Err(err) => match(err) {
///         PassablewordResult::TooCommon => status::BadRequest("Your should be more unique")
///     }
/// }
/// ```
pub fn check_uniqueness(password: &str) -> PassablewordResult {
    if PASSWORDS.contains(password) {
        Err(PasswordError::TooCommon)
    } else {
        Ok(())
    }
}

/// Check a password to make sure random enough that it would take a lot of effort to crack/guess.
/// This uses the awesome zxcvbn library behind the scenes.
///
/// # Example (using rocket.rs)
///
/// ```
/// match check_entropy(password) {
///     Ok() => status::Ok
///     Err(err) => match(err) {
///         PassablewordResult::TooSimple => status::BadRequest("Your should be more random")
///     }
/// }
/// ```
pub fn check_entropy(password: &str) -> PassablewordResult {
    match zxcvbn(password, &[]) {
        Ok(result) => {
            if result.score >= 3 {
                Ok(())
            } else {
                Err(PasswordError::TooSimple)
            }
        }
        Err(zxcvbn_error) => {
            match zxcvbn_error {
                ZxcvbnError::NonAsciiPassword => Err(PasswordError::NonAsciiPassword),
                _ => Err(PasswordError::InternalError),
            }
        }
    }
}

/// Check a password's length, uniqueness, and entropy all in a single call. This is a convenience
/// method and simply calls `check_length`, `check_uniqueness`, and `check_entropy` with the
/// password supplied.
///
/// # Example (using rocket.rs)
///
/// ```
/// match check_password(password) {
///     Ok() => status::Ok
///     Err(err) => match(err) {
///         PassablewordResult::TooShort => status::BadRequest("Your password should be longer than 8 characters"),
///         PassablewordResult::TooCommon => status::BadRequest("Your should be more unique"),
///         PassablewordResult::TooSimple => status::BadRequest("Your should be more random"),
///         PassablewordResult::InternalError => status::InternalServerError
///     }
/// }
/// ```
pub fn check_password(password: &str) -> PassablewordResult {
    check_length(password).and(check_uniqueness(password)).and(
        check_entropy(password),
    )
}

#[cfg(test)]
mod tests {
    use super::{check_entropy, check_length, check_password, check_uniqueness, PasswordError};

    #[test]
    fn it_validates_length() {
        let too_short = check_length("short");
        let long_enough = check_length("this is a long password");

        assert_eq!(too_short, Err(PasswordError::TooShort));
        assert_eq!(long_enough, Ok(()));
    }

    #[test]
    fn it_validates_uniqueness() {
        let too_common = check_uniqueness("password");
        let unique_enough = check_uniqueness("this is a unique password");

        assert_eq!(too_common, Err(PasswordError::TooCommon));
        assert_eq!(unique_enough, Ok(()));
    }

    #[test]
    fn it_validates_entropy() {
        let too_simple = check_entropy("NotTooRandom");
        let random_enough = check_entropy("Th1s iS a Sup3rR4ndom PassW0rd!");

        assert_eq!(too_simple, Err(PasswordError::TooSimple));
        assert_eq!(random_enough, Ok(()));
    }

    #[test]
    fn it_validates_a_password() {
        let too_short = check_password("short");
        let too_common = check_password("password");
        let too_simple = check_password("NotTooRandom");
        let ok_password = check_password("Th1s iS a Sup3rR4ndom PassW0rd!");

        assert_eq!(too_short, Err(PasswordError::TooShort));
        assert_eq!(too_common, Err(PasswordError::TooCommon));
        assert_eq!(too_simple, Err(PasswordError::TooSimple));
        assert_eq!(ok_password, Ok(()));
    }
}
