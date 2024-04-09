// SPDX-License-Identifier: BSD-3-Clause

use core::cmp::Ordering;

/// cursorsort sorts in place an array of any type that implements the
/// `PartialOrd` trait. It does so by using a modified quicksort algorithm
/// that uses cursor based partitioning and pivot selection.
///
/// It can sort in either ascending or descending order, depending on the
/// `descending` argument passed to the function.
///
/// This function works on arrays, slices, and vectors of any type satisfying
/// the trait requirement. If a type can be turned into a vector like a String
/// it will also be able to sort that.
pub fn cursorsort<T: PartialOrd>(arr: &mut [T], descending: bool) {
    // If the array is empty or of length 1, return it as is.
    if arr.len() <= 1 {
        return;
    }

    // Initialise the cursors.
    let mut cur1 = 0;
    let mut cur2 = arr.len() - 1;

    // Until the cursors are equal find a correctly placed element to use as
    // the pivot.
    while cur1 != cur2 {
        // Compare the cursors and the indexed elements and swap them if they
        // are not in the correct place.
        let mut swap = false;
        if !descending {
            // If the descending argument is false, sort ascending.
            match PartialOrd::partial_cmp(&arr[cur1], &arr[cur2]) {
                Some(Ordering::Greater) => {
                    if cur1 < cur2 {
                        swap = true
                    }
                }
                Some(Ordering::Less) => {
                    if cur1 > cur2 {
                        swap = true
                    }
                }
                _ => {}
            };
        } else {
            // If the descending argument is true, sort descending.
            match PartialOrd::partial_cmp(&arr[cur1], &arr[cur2]) {
                Some(Ordering::Greater) => {
                    if cur1 > cur2 {
                        swap = true
                    }
                }
                Some(Ordering::Less) => {
                    if cur1 < cur2 {
                        swap = true
                    }
                }
                _ => {}
            };
        }

        // Swap the elements at the cursors and the cursors themselves.
        if swap {
            arr.swap(cur1, cur2);
            (cur1, cur2) = (cur2, cur1);
        }

        // Incremement or decrement counter one based on its position.
        if cur1 < cur2 {
            cur1 += 1;
        } else {
            cur1 -= 1;
        }
    }

    // Recursively call cursorsort on the subarrays using the correctly placed
    // pivot element freom the while loop.
    cursorsort(&mut arr[..cur1], descending);
    cursorsort(&mut arr[cur1 + 1..], descending);
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending_arr_sort_ascending() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        cursorsort(&mut arr, false);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_descending_arr_sort_ascending() {
        let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        cursorsort(&mut arr, false);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_random_arr_sort_ascending() {
        let mut arr = [54, 24, 53, 6, 2, 2, 5, 6, 7, 2];
        cursorsort(&mut arr, false);
        assert_eq!(arr, [2, 2, 2, 5, 6, 6, 7, 24, 53, 54])
    }

    #[test]
    fn test_random2_arr_sort_ascending() {
        let mut arr = [123, 123, 1, 3, 3, 4, 45, 56, 643, 634];
        cursorsort(&mut arr, false);
        assert_eq!(arr, [1, 3, 3, 4, 45, 56, 123, 123, 634, 643])
    }

    #[test]
    fn test_alphabet_arr_sort_ascending() {
        let mut arr = ["b", "a", "c", "d", "e", "f", "g", "h", "i", "j"];
        cursorsort(&mut arr, false);
        assert_eq!(arr, ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"])
    }

    #[test]
    fn test_alphabet2_arr_sort_ascending() {
        let mut arr = ["q", "a", "c", "d", "r", "f", "z", "h", "i", "j"];
        cursorsort(&mut arr, false);
        assert_eq!(arr, ["a", "c", "d", "f", "h", "i", "j", "q", "r", "z"])
    }

    #[test]
    fn test_words_arr_sort_ascending() {
        let mut arr = [
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
        ];
        cursorsort(&mut arr, false);
        assert_eq!(
            arr,
            ["brown", "dog", "fox", "jumps", "lazy", "over", "quick", "the", "the"]
        )
    }

    #[test]
    fn test_words2_arr_sort_ascending() {
        let mut arr = [
            "lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet",
            "consectetur",
            "adipiscing",
            "elit",
        ];
        cursorsort(&mut arr, false);
        assert_eq!(
            arr,
            [
                "adipiscing",
                "amet",
                "consectetur",
                "dolor",
                "elit",
                "ipsum",
                "lorem",
                "sit"
            ]
        )
    }

    #[test]
    fn test_ascending_vec_sort_ascending() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        cursorsort(&mut vec, false);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_descending_vec_sort_ascending() {
        let mut vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        cursorsort(&mut vec, false);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_random_vec_sort_ascending() {
        let mut vec = vec![54, 24, 53, 6, 2, 2, 5, 6, 7, 2];
        cursorsort(&mut vec, false);
        assert_eq!(vec, vec![2, 2, 2, 5, 6, 6, 7, 24, 53, 54])
    }

    #[test]
    fn test_random2_vec_sort_ascending() {
        let mut vec = vec![123, 123, 1, 3, 3, 4, 45, 56, 643, 634];
        cursorsort(&mut vec, false);
        assert_eq!(vec, vec![1, 3, 3, 4, 45, 56, 123, 123, 634, 643])
    }

    #[test]
    fn test_alphabet_vec_sort_ascending() {
        let mut vec = vec!["b", "a", "c", "d", "e", "f", "g", "h", "i", "j"];
        cursorsort(&mut vec, false);
        assert_eq!(vec, vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"])
    }

    #[test]
    fn test_alphabet2_vec_sort_ascending() {
        let mut vec = vec!["q", "a", "c", "d", "r", "f", "z", "h", "i", "j"];
        cursorsort(&mut vec, false);
        assert_eq!(vec, vec!["a", "c", "d", "f", "h", "i", "j", "q", "r", "z"])
    }

    #[test]
    fn test_words_vec_sort_ascending() {
        let mut vec = vec![
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
        ];
        cursorsort(&mut vec, false);
        assert_eq!(
            vec,
            vec!["brown", "dog", "fox", "jumps", "lazy", "over", "quick", "the", "the"]
        )
    }

    #[test]
    fn test_words2_vec_sort_ascending() {
        let mut vec = vec![
            "lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet",
            "consectetur",
            "adipiscing",
            "elit",
        ];
        cursorsort(&mut vec, false);
        assert_eq!(
            vec,
            vec![
                "adipiscing",
                "amet",
                "consectetur",
                "dolor",
                "elit",
                "ipsum",
                "lorem",
                "sit"
            ]
        )
    }

    #[test]
    fn test_empty_string_sort_ascending() {
        let string = String::new();
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, false);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "");
    }

    #[test]
    fn test_single_char_string_sort_ascending() {
        let string = String::from("a");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, false);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "a");
    }

    #[test]
    fn test_sorted_string_sort_ascending() {
        let string = String::from("abcdef");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, false);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "abcdef");
    }

    #[test]
    fn test_reverse_sorted_string_sort_ascending() {
        let string = String::from("fedcba");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, false);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "abcdef");
    }

    #[test]
    fn test_random_string_sort_ascending() {
        let string = String::from("dcab");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, false);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "abcd");
    }

    #[test]
    fn test_string_with_duplicates_sort_ascending() {
        let string = String::from("dadbebcec");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, false);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "abbccddee");
    }

    #[test]
    fn test_string_with_whitespace_sort_ascending() {
        let string = String::from("hello world");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, false);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, " dehllloorw");
    }

    #[test]
    fn test_ascending_arr_sort_descending() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        cursorsort(&mut arr, true);
        assert_eq!(arr, [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_descending_arr_sort_descending() {
        let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        cursorsort(&mut arr, true);
        assert_eq!(arr, [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_random_arr_sort_descending() {
        let mut arr = [54, 24, 53, 6, 2, 2, 5, 6, 7, 2];
        cursorsort(&mut arr, true);
        assert_eq!(arr, [54, 53, 24, 7, 6, 6, 5, 2, 2, 2])
    }

    #[test]
    fn test_random2_arr_sort_descending() {
        let mut arr = [123, 123, 1, 3, 3, 4, 45, 56, 643, 634];
        cursorsort(&mut arr, true);
        assert_eq!(arr, [643, 634, 123, 123, 56, 45, 4, 3, 3, 1])
    }

    #[test]
    fn test_alphabet_arr_sort_descending() {
        let mut arr = ["b", "a", "c", "d", "e", "f", "g", "h", "i", "j"];
        cursorsort(&mut arr, true);
        assert_eq!(arr, ["j", "i", "h", "g", "f", "e", "d", "c", "b", "a"])
    }

    #[test]
    fn test_alphabet2_arr_sort_descending() {
        let mut arr = ["q", "a", "c", "d", "r", "f", "z", "h", "i", "j"];
        cursorsort(&mut arr, true);
        assert_eq!(arr, ["z", "r", "q", "j", "i", "h", "f", "d", "c", "a"])
    }

    #[test]
    fn test_words_arr_sort_descending() {
        let mut arr = [
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
        ];
        cursorsort(&mut arr, true);
        assert_eq!(
            arr,
            ["the", "the", "quick", "over", "lazy", "jumps", "fox", "dog", "brown"]
        )
    }

    #[test]
    fn test_words2_arr_sort_descending() {
        let mut arr = [
            "lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet",
            "consectetur",
            "adipiscing",
            "elit",
        ];
        cursorsort(&mut arr, true);
        assert_eq!(
            arr,
            [
                "sit",
                "lorem",
                "ipsum",
                "elit",
                "dolor",
                "consectetur",
                "amet",
                "adipiscing",
            ]
        )
    }

    #[test]
    fn test_ascending_vec_sort_descending() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        cursorsort(&mut vec, true);
        assert_eq!(vec, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_descending_vec_sort_descending() {
        let mut vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        cursorsort(&mut vec, true);
        assert_eq!(vec, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_random_vec_sort_descending() {
        let mut vec = vec![54, 24, 53, 6, 2, 2, 5, 6, 7, 2];
        cursorsort(&mut vec, true);
        assert_eq!(vec, vec![54, 53, 24, 7, 6, 6, 5, 2, 2, 2])
    }

    #[test]
    fn test_random2_vec_sort_descending() {
        let mut vec = vec![123, 123, 1, 3, 3, 4, 45, 56, 643, 634];
        cursorsort(&mut vec, true);
        assert_eq!(vec, vec![643, 634, 123, 123, 56, 45, 4, 3, 3, 1])
    }

    #[test]
    fn test_alphabet_vec_sort_descending() {
        let mut vec = vec!["b", "a", "c", "d", "e", "f", "g", "h", "i", "j"];
        cursorsort(&mut vec, true);
        assert_eq!(vec, vec!["j", "i", "h", "g", "f", "e", "d", "c", "b", "a"])
    }

    #[test]
    fn test_alphabet2_vec_sort_descending() {
        let mut vec = vec!["q", "a", "c", "d", "r", "f", "z", "h", "i", "j"];
        cursorsort(&mut vec, true);
        assert_eq!(vec, vec!["z", "r", "q", "j", "i", "h", "f", "d", "c", "a"])
    }

    #[test]
    fn test_words_vec_sort_descending() {
        let mut vec = vec![
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
        ];
        cursorsort(&mut vec, true);
        assert_eq!(
            vec,
            vec!["the", "the", "quick", "over", "lazy", "jumps", "fox", "dog", "brown"]
        )
    }

    #[test]
    fn test_words2_vec_sort_descending() {
        let mut vec = vec![
            "lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet",
            "consectetur",
            "adipiscing",
            "elit",
        ];
        cursorsort(&mut vec, true);
        assert_eq!(
            vec,
            vec![
                "sit",
                "lorem",
                "ipsum",
                "elit",
                "dolor",
                "consectetur",
                "amet",
                "adipiscing",
            ]
        )
    }

    #[test]
    fn test_empty_string_sort_descending() {
        let string = String::new();
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, true);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "");
    }

    #[test]
    fn test_single_char_string_sort_descending() {
        let string = String::from("a");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, true);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "a");
    }

    #[test]
    fn test_sorted_string_sort_descending() {
        let string = String::from("abcdef");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, true);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "fedcba");
    }

    #[test]
    fn test_reverse_sorted_string_sort_descending() {
        let string = String::from("fedcba");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, true);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "fedcba");
    }

    #[test]
    fn test_random_string_sort_descending() {
        let string = String::from("dcab");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, true);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "dcba");
    }

    #[test]
    fn test_string_with_duplicates_sort_descending() {
        let string = String::from("dadbebcec");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, true);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "eeddccbba");
    }

    #[test]
    fn test_string_with_whitespace_sort_descending() {
        let string = String::from("hello world");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort(&mut bytes, true);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "wroolllhed ");
    }
}
