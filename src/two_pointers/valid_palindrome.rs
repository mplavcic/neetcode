/*

    EASY

    Is Palindrome
    Given a string s, return true if it is a palindrome, otherwise return false.
    A palindrome is a string that reads the same forward and backward. It is also
    case-insensitive and ignores all non-alphanumeric characters.

    Example 1:
    Input: s = "Was it a car or a cat I saw?"
    Output: true
    Explanation: After considering only alphanumerical characters we have "wasitacaroracatisaw", which is a palindrome.

    Example 2:
    Input: s = "tab a cat"
    Output: false
    Explanation: "tabacat" is not a palindrome.

    Constraints:
    1 <= s.length <= 1000
    s is made up of only printable ASCII characters.

*/

fn is_palindrome(s: &str) -> bool {
    let s: String = s
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let s_rev: String = s.chars().rev().collect();

    s == s_rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neetcode_example_1() {
        assert!(is_palindrome("Was it a car or a cat I Saw"));
    }

    #[test]
    fn neetcode_example_2() {
        let s = "";
        assert!(is_palindrome(s));
    }

    #[test]
    fn empty_string() {
        assert!(is_palindrome(""));
    }

    #[test]
    fn single_character() {
        assert!(is_palindrome("a"));
        assert!(is_palindrome("Z"));
    }

    #[test]
    fn simple_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("Madam"));
        assert!(is_palindrome("A man a plan a canal Panama"));
    }

    #[test]
    fn palindrome_with_special_characters() {
        assert!(is_palindrome("No 'x' in Ni'x'on"));
    }

    #[test]
    #[should_panic]
    fn non_palindrome() {
        assert!(is_palindrome("hello"));
        assert!(is_palindrome("tab a cat"));
    }

    #[test]
    fn palindrome_with_numbers() {
        assert!(is_palindrome("12321"));
    }

    #[test]
    fn mixed_case() {
        assert!(is_palindrome("Able was I ere I saw Elba"));
    }
}
