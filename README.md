# Operating System 110 Exercism
## Scrabble Score Problem

According to [Wikipedia](https://en.wikipedia.org/wiki/Scrabble), Scrabble is a word game in which two to four players score points by placing tiles bearing a single letter onto a board divided into a 15×15 grid of squares. The tiles must form words that, in crossword fashion, read left to right in rows or downward in columns, and be included in a standard dictionary or lexicon. The values obtained from each letter are as follows...

```
Letter                           Value
A, E, I, O, U, L, N, R, S, T       1
D, G                               2
B, C, M, P                         3
F, H, V, W, Y                      4
K                                  5
J, X                               8
Q, Z                               10
```

## Problem

This problem aims to find a solution that can calculate the value of each letter in the word. For example..
**FATHURRAHMAN**

**FATHURRAHMAN** should be scored as worth 14 points:
+ 4 points for F
+ 1 point for A, three times
+ 1 point for T
+ 1 point for H, twice
+ 1 point for U
+ 1 point for R, twice
+ 3 points for M
+ 1 point for N

And to total:
+ 4 + 3 * 1 + 1 + 2 * 1 + 1 + 2 * 1 + 3 + 1
+ = 4 + 3 + 1 + 2 + 1 + 2 + 3 + 1
+ = 13 + 4
+ = 17

# SOLUTION

The way to solve this problem is by adding up each value of each letter contained in the word

## Score

This function is used to define the value of each letter in the scrabble game as described above using HashMap, It is expected that the values of each letter will be made in advance so that the values can be taken. Then reading the characters to be tested, separated by character, then each character is determined to be valued from the HashMap vector, then all the letters are summed in the form of integers.

Here's the full code.

```rust
/// Compute the Scrabble score for a word.

use std::collections::HashMap;

pub fn score(word: &str) -> u64 {
//    unimplemented!("Score {} in Scrabble.", word);
    let scores: HashMap<_, _> = vec![
        ('a', 1),
        ('b', 3),
        ('c', 3),
        ('d', 2),
        ('e', 1),
        ('f', 4),
        ('g', 2),
        ('h', 4),
        ('i', 1),
        ('j', 8),
        ('k', 5),
        ('l', 1),
        ('m', 3),
        ('n', 1),
        ('o', 1),
        ('p', 3),
        ('q', 10),
        ('r', 1),
        ('s', 1),
        ('t', 1),
        ('u', 1),
        ('v', 4),
        ('w', 4),
        ('x', 8),
        ('y', 4),
        ('z', 10),
    ].into_iter()
        .collect();

    word.to_lowercase()
        .chars()
        .filter_map(|c| scores.get(&c))
        .sum()
}
```
