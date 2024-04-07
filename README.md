# cursorsort

CursorSort is an implementation of the QuickSort algorithm using only cursors,
to both partition and select the accurately placed pivot element. It does so by
choosing the initial and last elements of the array as its cursors and then in a
loop swapping elements that are either greater than the two indexes the cursors
point to and the first cursor is less than the second or vice versa.

This sorting algorithm works on any type implementing the `PartialOrd` and `Copy`
traits - the test cases cover arrays of integers, letters and words.

## Overview

The algorithm essentially works as follows:

1. Choose the first and last elements of the array as cursors

1. While the first cursor is not equal to the second loop over steps 3-5

1. If the element at the first cursor is greater than the element at the second
   cursor **and** the first cursor is smaller than the second or vice versa
   a. The first element is smaller but the first cursor is bigger

1. Assuming step 3 is true Swap the elements at the first and last cursor and
   the cursors themselves, otherwise do nothing.

1. If the first counter is less than the second counter incremenet it, otherwise
   decrement it.

1. This correctly places a pivot element and from the cursors (now euqual) we
   recursively call the function on the left and right halves of the array.

## Installation

To use this algorithm in your project, run the following command:

```sh
cargo add cursorsort
```
