// SPDX-License-Identifier: BSD-3-Clause

/// cursorsort sorts in place an array of any type that implements the
/// `PartialOrd` and `Copy` traits. It does so by using a modified quiksort
/// algorithm that uses cursor based partitioning and pivot selection.
///
/// NOTE: This function is specific to arrays and will mutate them inplace.
pub fn cursorsort<T: PartialOrd + Copy>(arr: &mut [T]) {
    // If the array is empty or of length 1, return it as is.
    if arr.len() <= 1 {
        return;
    }

    // Initialise the cursors
    let mut cur1 = 0;
    let mut cur2 = arr.len() - 1;

    // Until the cursors are equal find a correctly placed element to use as
    // the pivot.
    while cur1 != cur2 {
        // Compare the cursors and the indexed elements and swap them if they
        // are not in the correct place.
        if (arr[cur1] > arr[cur2] && cur1 < cur2) || (arr[cur1] < arr[cur2] && cur1 > cur2) {
            arr.swap(cur1, cur2);
            (cur1, cur2) = (cur2, cur1);
        }
        if cur1 < cur2 {
            cur1 += 1;
        } else {
            cur1 -= 1;
        }
    }

    // Recursively call cursorsort on the subarrays using the correctly placed
    // pivot element freom the while loop.
    cursorsort(&mut arr[..cur1]);
    cursorsort(&mut arr[cur1 + 1..]);
}

/// cursorsort_vec sorts in place a vector of any type that implements the
/// `PartialOrd` and `Copy` traits. It does so by using a modified quiksort
/// algorithm that uses cursor based partitioning and pivot selection.
///
/// NOTE: This function is specific to vectors and will mutate them inplace.
pub fn cursorsort_vec<T: PartialOrd + Copy>(vec: &mut Vec<T>) {
    // If the array is empty or of length 1, return it as is.
    if vec.len() <= 1 {
        return;
    }

    // Initialise the cursors
    let mut cur1 = 0;
    let mut cur2 = vec.len() - 1;

    // Until the cursors are equal find a correctly placed element to use as
    // the pivot.
    while cur1 != cur2 {
        // Compare the cursors and the indexed elements and swap them if they
        // are not in the correct place.
        if (vec[cur1] > vec[cur2] && cur1 < cur2) || (vec[cur1] < vec[cur2] && cur1 > cur2) {
            vec.swap(cur1, cur2);
            (cur1, cur2) = (cur2, cur1);
        }
        if cur1 < cur2 {
            cur1 += 1;
        } else {
            cur1 -= 1;
        }
    }

    // Recursive call cursorsort on the two sub-slices
    cursorsort(&mut vec[..cur1]);
    cursorsort(&mut vec[cur1 + 1..]);
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ascending_arr() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        cursorsort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_descending_arr() {
        let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        cursorsort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_random_arr() {
        let mut arr = [54, 24, 53, 6, 2, 2, 5, 6, 7, 2];
        cursorsort(&mut arr);
        assert_eq!(arr, [2, 2, 2, 5, 6, 6, 7, 24, 53, 54])
    }

    #[test]
    fn test_random2_arr() {
        let mut arr = [123, 123, 1, 3, 3, 4, 45, 56, 643, 634];
        cursorsort(&mut arr);
        assert_eq!(arr, [1, 3, 3, 4, 45, 56, 123, 123, 634, 643])
    }

    #[test]
    fn test_alphabet_arr() {
        let mut arr = ["b", "a", "c", "d", "e", "f", "g", "h", "i", "j"];
        cursorsort(&mut arr);
        assert_eq!(arr, ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"])
    }

    #[test]
    fn test_alphabet2_arr() {
        let mut arr = ["q", "a", "c", "d", "r", "f", "z", "h", "i", "j"];
        cursorsort(&mut arr);
        assert_eq!(arr, ["a", "c", "d", "f", "h", "i", "j", "q", "r", "z"])
    }

    #[test]
    fn test_words_arr() {
        let mut arr = [
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
        ];
        cursorsort(&mut arr);
        assert_eq!(
            arr,
            ["brown", "dog", "fox", "jumps", "lazy", "over", "quick", "the", "the"]
        )
    }

    #[test]
    fn test_words2_arr() {
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
        cursorsort(&mut arr);
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
    fn test_ascending_vec() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        cursorsort_vec(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_descending_vec() {
        let mut vec = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        cursorsort_vec(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_random_vec() {
        let mut vec = vec![54, 24, 53, 6, 2, 2, 5, 6, 7, 2];
        cursorsort_vec(&mut vec);
        assert_eq!(vec, vec![2, 2, 2, 5, 6, 6, 7, 24, 53, 54])
    }

    #[test]
    fn test_random2_vec() {
        let mut vec = vec![123, 123, 1, 3, 3, 4, 45, 56, 643, 634];
        cursorsort_vec(&mut vec);
        assert_eq!(vec, vec![1, 3, 3, 4, 45, 56, 123, 123, 634, 643])
    }

    #[test]
    fn test_alphabet_vec() {
        let mut vec = vec!["b", "a", "c", "d", "e", "f", "g", "h", "i", "j"];
        cursorsort_vec(&mut vec);
        assert_eq!(vec, vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"])
    }

    #[test]
    fn test_alphabet2_vec() {
        let mut vec = vec!["q", "a", "c", "d", "r", "f", "z", "h", "i", "j"];
        cursorsort_vec(&mut vec);
        assert_eq!(vec, vec!["a", "c", "d", "f", "h", "i", "j", "q", "r", "z"])
    }

    #[test]
    fn test_words_vec() {
        let mut vec = vec![
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
        ];
        cursorsort_vec(&mut vec);
        assert_eq!(
            vec,
            vec!["brown", "dog", "fox", "jumps", "lazy", "over", "quick", "the", "the"]
        )
    }

    #[test]
    fn test_words2_vec() {
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
        cursorsort_vec(&mut vec);
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
    fn test_empty_string() {
        let string = String::new();
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort_vec(&mut bytes);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "");
    }

    #[test]
    fn test_single_char_string() {
        let string = String::from("a");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort_vec(&mut bytes);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "a");
    }

    #[test]
    fn test_sorted_string() {
        let string = String::from("abcdef");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort_vec(&mut bytes);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "abcdef");
    }

    #[test]
    fn test_reverse_sorted_string() {
        let string = String::from("fedcba");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort_vec(&mut bytes);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "abcdef");
    }

    #[test]
    fn test_random_string() {
        let string = String::from("dcab");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort_vec(&mut bytes);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "abcd");
    }

    #[test]
    fn test_string_with_duplicates() {
        let string = String::from("dadbebcec");
        let mut bytes: Vec<u8> = string.bytes().collect();
        cursorsort_vec(&mut bytes);
        let sorted_string = String::from_utf8(bytes).unwrap();
        assert_eq!(sorted_string, "abbccddee");
    }
}
