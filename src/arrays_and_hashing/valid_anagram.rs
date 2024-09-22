/*

   Is Anagram:
   Given two strings s and t, return true if the two strings are anagrams of each other, otherwise return false.
   An anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.

   Example 1:
   Input: s = "racecar", t = "carrace"
   Output: true

   Example 2:
   Input: s = "jar", t = "jam"
   Output: false

   Constraints: s and t consist of lowercase English letters.

*/

pub fn is_anagram(str1: &str, str2: &str) -> bool {
    let mut vec1: Vec<char> = str1.chars().collect();
    let mut vec2: Vec<char> = str2.chars().collect();

    vec1.sort();
    vec2.sort();

    vec1 == vec2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let str1 = "racecar";
        let str2 = "carrace";
        assert_eq!(is_anagram(str1, str2), true);
    }

    #[test]
    fn test2() {
        let str1 = "jar";
        let str2 = "jam";
        assert_eq!(is_anagram(str1, str2), false);
    }
}
