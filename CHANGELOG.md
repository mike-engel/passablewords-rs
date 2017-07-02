# 1.0.0
> 2017-07-01

The 1.0.0 release of passablewords for rust!

- Added the benchmarks to the README
- Attempted to store the passwords in a rust file, but found performance to drop,
  so this will continue reading from the text file.
- Dependency updates
- New error `NonAsciiPassword` for catching a zxcvbn error

# 0.1.1
> 2017-04-02

#### Bug fixes
- fixed the example bloc in the crate docs

# 0.1.0
> 2017-04-02

The initial release of passablewords for rust. Since this is 0.1.0, things could
change in the future!

- Checks for password length (must be at least 8 characters long)
- Checks against a list of the 1,000,000 most common passwords
- Checks the randomness of the password in an attempt to deter hackers

#### Roadmap to 1.0.0
- Gather any feedback to make sure the API is sane
- Attempt to increase performance (finishes in < 2ms right now)
