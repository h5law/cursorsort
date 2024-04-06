# cursorsort

CursorSort is an implementation of the QuickSort algorithm using only cursors,
to both partition and select the accurately placed pivot element. It does so by
choosing the initial and last elements of the array as its cursors and then in a
loop swapping elements that are either greater than the two indexes the cursors
point to and the first cursor is less than the second or vice versa.

This sorting algorithm works on any type implementing the `PartialOrd` and `Copy`
traits - the test cases cover arrays of integers, letters and words.

## Installation

To use this algorithm in your project, run the following command:

```sh
cargo add cursorsort
```
