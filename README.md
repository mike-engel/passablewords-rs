# passablewords

---

# DEPRECATED

I'm not planning on maintaining this any longer. Please just use [zxcvbn](https://github.com/shssoichiro/zxcvbn-rs) instead.

---

`passablewords` is a password validation library which checks a password against a million of
the most common as well as it's ability to be cracked.

If you're asking why use `passablewords` over [`zxcvbn`](https://github.com/shssoichiro/zxcvbn-rs), it's because `passablewords` checks
a password against 1,000,000 of the most common passwords. `zxcvbn` only checks 30,000.
`zxcvbn` is a great tool, however, and `passablewords` uses it to check the entropy of a given
password to make sure it's random enough on top of being unique enough. If you are ok with the
top 30,000 most common passwords, then you should probably use `zxcvbn`. If you want a little
extra, consider `passablewords`.

While you're free to use any of the public methods, using the `check_password` function is
recommended since that checks for length, uniqueness, and entropy all within a single call.

It's also important to note that this is provided as-is and doesn't prevent an attacker from
gaining access to, decrypting, or guessing your user's passwords. It just makes it a little
harder.

# Installing `passablewords`

`passablewords` can be added to your project via the `dependencies` section of your `cargo.toml`
file.

```
[dependencies]
passablewords = "1"
```

# Using `passablewords`

For more information on how to use this library, please refer to the [docs](https://docs.rs/passablewords).

Generally, however, you would use it similar to this example.

```rust
extern crate passablewords;

use passablewords::{check_password, PassablewordResult};

fn main() {
    match check_password(password) {
        Ok() => println!("That password is probably pretty good!")
        Err(err) => match(err) {
            PassablewordResult::TooShort => println!("Your password should be longer than 8 characters"),
            PassablewordResult::TooCommon => println!("Your should be more unique"),
            PassablewordResult::TooSimple => println!("Your should be more random"),
            PassablewordResult::NonAsciiPassword => println!("Your password should only contain ASCII characters"),
            PassablewordResult::InternalError => println!
        }
    }
}
```

## How fast is it?

Here are the benchmarks running on a 2017 MacBook Pro with 2.3GHz i5. It's pretty darn fast!

```
test bench_check_common_password ... bench:          26 ns/iter (+/- 5)
test bench_check_ok_password     ... bench:   1,497,032 ns/iter (+/- 435,657)
test bench_check_short_password  ... bench:           1 ns/iter (+/- 0)
test bench_check_simple_password ... bench:     166,063 ns/iter (+/- 70,071)
```

# Developing

Thanks to the Rust community, getting this project up and running to begin contributing to is
pretty easy!

First you'll need to have rust installed (probably stable, but nightly would be fine too. In
fact, you need nightly to run the benchmarks). I recommend [rustup](https://rustup.rs), but
you're free to install rust however you like.

Since Rust comes with cargo installed, you should download the cargo dependencies next.

```sh
cargo update
```

It would be a good idea at this point to make sure all the tests pass so let's run the tests.

```sh
cargo test
```

Finally, make any changes you want and submit a [pr](https://github.com/mike-engel/passablewords-rs/pulls/new). Thanks in advance!

# [Code of Conduct](code_of_conduct.md)

# [License](LICENSE.md)

The code for this library is licensed under the MIT license.

The list of passwords is licensed under the [Creative Commons Attribution ShareAlike 3.0](https://creativecommons.org/licenses/by-sa/3.0/) license.

Thanks to the [SecLists](https://www.owasp.org/index.php/Projects/OWASP_SecLists_Project) project for the list.

# [Changelog](CHANGELOG.md)
