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