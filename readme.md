F(x): FizzBuzz by Cargo
=======================

[/r/learnprogramming/](http://www.reddit.com/r/learnprogramming/) has been chattering about FizzBuzz all week long for no apparent reason, so I thought I would just go ahead and solve this problem once and for all. This version will work for FizzBuzz A and B, for arbitrary values of A and B (canonically 3 and 5, respectively).

## Usage

* `Fx::new(n)` creates a new enum for `n`, with matching condition A as `n % 3 == 0` and matching condition B as `n % 5 == 0`
* `Fx::arbitrary(n, |n| n % 3 == 0, |n| n % 5 == 0)` does exactlty the same thing, except you can change those predicates out for anything you like.
