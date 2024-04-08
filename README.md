# cursorsort

CursorSort is an implementation of the QuickSort algorithm using only cursors,
to both partition and select the accurately placed pivot element. It does so by
choosing the initial and last elements of the array as its cursors and then in a
loop swapping elements that are either greater than the two indexes the cursors
point to and the first cursor is less than the second or vice versa.

This sorting algorithm works on any type implementing the `PartialOrd` and `Copy`
traits - the test cases cover `array`s and `Vec`tors of integers, letters and words,
as well as `String`s.

## Overview

The algorithm essentially works as follows:

1. Choose the first and last elements of the array as cursors

1. While the first cursor is not equal to the second loop over steps 3-5

1. If the element at the first cursor is greater than the element at the second
   cursor **and** the first cursor is smaller than the second or vice versa
   a. The first element is smaller but the first cursor is larger

1. Assuming step 3 is true Swap the elements at the first and last cursor and
   the cursors themselves, otherwise do nothing.

1. If the first counter is less than the second counter increment it, otherwise
   decrement it.

1. This correctly places a pivot element and from the cursors (now equal) we
   recursively call the function on the left and right halves of the array.

## Installation

To use this algorithm in your project, run the following command:

```sh
cargo add cursorsort
```

## Usage

Any type implementing the `PartialOrd` and `Copy` traits can be used, with
the `cursorsort` function for arrays/slices and vectors. It operates on them in
place and requires no `std` library to function.

If something can be converted into a `Vec` it will be able to be sorted
(provided the trait requirements are met).

```rust
use cursorsort::cursorsort;

fn main() {
    // For arrays:

    let mut array = [5, 3, 2, 4, 1];
    cursorsort(&mut array, false);
    println!("Sorted array: {:?}", array); // [ 1, 2, 3, 4, 5 ]

    // For Vecs:

    let mut vector = vec![1, 2, 3, 4, 5];
    cursorsort(&mut vector, true);
    println!("Sorted vector: {:?}", vector); // [ 5, 4, 3, 2, 1 ]

    // For Strings:

    // Convert a String to a Vec<u8>
    let mut bytes = String::from("hello world").into_bytes();
    // Sort the vector in place
    cursorsort(&mut bytes, false);
    // Convert the sorted Vec<u8> back into a String
    let sorted_string = String::from_utf8(bytes).expect("Invalid UTF-8");
    println!("Sorted string: {:?}", sorted_string); // " dehllloorw"
}
```
