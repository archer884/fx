F(x): FizzBuzz by Cargo
=======================

[/r/learnprogramming/](http://www.reddit.com/r/learnprogramming/) has been chattering about FizzBuzz all week long for no apparent reason, so I thought I would just go ahead and solve this problem once and for all. This version will work for FizzBuzz A and B, for arbitrary values of A and B (canonically 3 and 5, respectively). Later enhancements should work for an arbitrary number of values as well, but you are just gonna have to take what you can get this time around, because I'm not aware of a `params` keyword in Rust. Actually, if I'm going to do that, I'm going to have to stop using the algebraic type for the different states, because that's not something you can futz with at compile time in a library...
