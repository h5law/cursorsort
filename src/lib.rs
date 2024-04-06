// SPDX-License-Identifier: BSD-3-Clause

/// cursorsort is a self partitioning quicksort implementation using cursors
/// to find the pivot points and rearrange them in a single loop.
/// It works on any type implementing the `PartialOrd` and `Copy` triats.
pub fn cursorsort<T: PartialOrd + Copy>(arr: &mut [T]) -> &mut [T] {
    if arr.len() <= 1 {
        return arr;
    }
    let mut cursor1 = 0;
    let mut cursor2 = arr.len() - 1;

    while cursor1 != cursor2 {
        if (arr[cursor1] > arr[cursor2] && cursor1 < cursor2)
            || (arr[cursor1] < arr[cursor2] && cursor1 > cursor2)
        {
            arr.swap(cursor1, cursor2);
            (cursor1, cursor2) = (cursor2, cursor1);
        }
        if cursor1 < cursor2 {
            cursor1 += 1;
        } else {
            cursor1 -= 1;
        }
    }

    cursorsort(&mut arr[..cursor1]);
    cursorsort(&mut arr[cursor1 + 1..]);

    return arr;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_presorted() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let sorted = cursorsort(&mut arr);
        assert_eq!(sorted, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_descending() {
        let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let sorted = cursorsort(&mut arr);
        assert_eq!(sorted, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_random() {
        let mut arr = [54, 24, 53, 6, 2, 2, 5, 6, 7, 2];
        let sorted = cursorsort(&mut arr);
        assert_eq!(sorted, [2, 2, 2, 5, 6, 6, 7, 24, 53, 54])
    }

    #[test]
    fn test_random2() {
        let mut arr = [123, 123, 1, 3, 3, 4, 45, 56, 643, 634];
        let sorted = cursorsort(&mut arr);
        assert_eq!(sorted, [1, 3, 3, 4, 45, 56, 123, 123, 634, 643])
    }

    #[test]
    fn test_alphabet() {
        let mut arr = ["b", "a", "c", "d", "e", "f", "g", "h", "i", "j"];
        let sorted = cursorsort(&mut arr);
        assert_eq!(sorted, ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"])
    }

    #[test]
    fn test_alphabet2() {
        let mut arr = ["q", "a", "c", "d", "r", "f", "z", "h", "i", "j"];
        let sorted = cursorsort(&mut arr);
        assert_eq!(sorted, ["a", "c", "d", "f", "h", "i", "j", "q", "r", "z"])
    }

    #[test]
    fn test_words() {
        let mut arr = [
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
        ];
        let sorted = cursorsort(&mut arr);
        assert_eq!(
            sorted,
            ["brown", "dog", "fox", "jumps", "lazy", "over", "quick", "the", "the"]
        )
    }

    #[test]
    fn test_words2() {
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
        let sorted = cursorsort(&mut arr);
        assert_eq!(
            sorted,
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
}
