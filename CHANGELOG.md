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
