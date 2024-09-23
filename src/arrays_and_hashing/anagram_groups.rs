/*
    MEDIUM

    Anagram Groups:
    Given an array of strings strs, group all anagrams together into sublists.
    You may return the output in any order. An anagram is a string that contains
    the exact same characters as another string, but the order of the characters can be different.

    Example 1:
    Input: strs = ["act","pots","tops","cat","stop","hat"]
    Output: [["hat"],["act", "cat"],["stop", "pots", "tops"]]

    Example 2:
    Input: strs = ["x"]
    Output: [["x"]]

    Example 3:
    Input: strs = [""]
    Output: [[""]]

    Constraints:
    1 <= strs.length <= 1000.
    0 <= strs[i].length <= 100
    strs[i] is made up of lowercase English letters.

*/

use std::collections::HashMap;

fn group_anagrams(strs: &[&str]) -> Vec<Vec<String>> {
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

    strs.iter().for_each(|str| {
        let mut sorted_str: Vec<char> = str.chars().collect();
        sorted_str.sort();
        let key: String = sorted_str.iter().collect();
        anagram_map
            .entry(key)
            .or_insert(Vec::new())
            .push(str.to_string());
    });

    anagram_map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs = ["act", "pots", "tops", "cat", "stop", "hat"];

        assert_eq!(
            group_anagrams(&strs),
            vec!(
                vec!("hat"),
                vec!("act", "cat"),
                vec!("stop", "pots", "tops")
            )
        );
    }

    #[test]
    fn test2() {
        let strs = ["x"];
        assert_eq!(group_anagrams(&strs), vec!(vec!("x")));
    }

    #[test]
    fn test3() {
        let strs = [""];
        assert_eq!(group_anagrams(&strs), vec!(vec!("")));
    }
}
