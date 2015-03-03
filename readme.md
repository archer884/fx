F(x): FizzBuzz by Cargo
=======================

[/r/learnprogramming/](http://www.reddit.com/r/learnprogramming/) has been chattering about FizzBuzz all week long for no apparent reason, so I thought I would just go ahead and solve this problem once and for all. This version will work for FizzBuzz A and B for arbitrary values of A and B (canonically 3 and 5, respectively).

## Usage

* `Fx::new(n)` creates a new enum for `n`, with matching condition A as `n % 3 == 0` and matching condition B as `n % 5 == 0`
* `Fx::arbitrary(n, |n| n % 3 == 0, |n| n % 5 == 0)` does exactly the same thing, except you can change those predicates out for anything you like.

> For full code samples, clone and build documentation using `cargo doc`

## Possible enhancements

I'd like to figure out a way this library can help you (easily; you can already do it with the library if you really want) find the sum of the fizz buzz values. Finding the sum of their inverse is cake, of course. Might need to add a new type for that, since it would be a breaking change to the current one.
