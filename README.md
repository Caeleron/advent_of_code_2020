# Advent of Code 2020
A repo containing my solutions to the Advent of Code 2020 puzzles and challenges. I also will make some comments on the
puzzles and what I did to solve them.

## Day 1 - Report Repair
### Part 1
This problem is deceptively simple. It's easy to use the naive solution where you test all the combinations until you
find the right pair, but this would run in quadratic time! Surely we can do better than this. The key here is to realize
that if you have one integer, you can easily determine what the other number should be with a single subtraction. All
that leaves is to remember which number is needed, so we can just throw each number that has been looked at into a set
after checking if the other number has been seen before. This results in a much better linear time algorithm. Not too
hard, but still a good exercise.

### Part 2
And now things get interesting. The naive solution is now cubic, so a clever solution is just begging to be found. But
how much better can we get it? Is linear time possible? I haven't found a reason why it can't be, but I can't find a way
to do it. If I do come across a way, I might code it and throw it in here, but for now I simply adapted Part 1's
solution to be generic to any sum as opposed to just 2020. Since that runs in linear time, running it once for every
input makes it quadratic. It makes me wonder what the running time of the generic n-sum problem is...

## Day 2 - Password Philosophy
### Part 1
This was a simple exercise in string manipulation. Most of the code is breaking up the input string into the correct
pieces before parsing them into the correct types. Using `filter_map` to toss out lines that don't parse correctly made
things much easier, so I could simply apply `?` operator on everything and the FilterMap iterator will pull the value
out of the Option for me.

### Part 2
Surprisingly, this didn't take much modification. I only changed the name of some struct fields within the parser. Then,
I replaced the character counting code with index-and-check code, and it worked like a charm. 

## Day 3 - Toboggan Trajectory
### Part 1
Somehow, the tools provided by Rust has made this puzzle incredibly easy. In fact, the code I wrote for this part is
purely functional except for the fact that I threw the final count in a variable before printing it out. I could have
put that entire thing in the `println!()` macro, but it was easier to throw it into a variable when writing it. Also,
using an iterator over the lines of input as an iterator over the y coordinate proved to be helpful here and later.

### Part 2
I split out my code from part 1 into its own function. I could then make it more general and able to handle all sorts of
different slopes. Getting the lines iterator to perform a "move down n" operation took a bit of experimentation before I
was able to get it just right. The `step_by()` function that the Iterator types provide really helped here.

## Day 4 - Passport Processing
The two parts are similar enough, so I won't make separate sections for them. The name of the game is input validation,
and there is plenty to be had here. I got to experiment with a few tricks; namely, folding a partially constructed
struct and a list of completed structs, and using `Option` to determine if a field was valid or even supplied.