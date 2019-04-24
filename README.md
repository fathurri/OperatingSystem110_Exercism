# OperatingSystem110_Exercism
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
```FATHURRAHMAN````

"FATHURRAHMAN" should be scored as worth 14 points:
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
