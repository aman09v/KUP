pub fn is_rotation(str1: &str, str2: &str) -> bool {
    let len1 = str1.len();
    let len2 = str2.len();
    let string1: Vec<char> = str1.chars().collect();
    let string2: Vec<char> = str2.chars().collect();
    if len1 != len2 {
        return false;
    }
    //the longest prefix suffix value for pattern
    let mut longest_prefix_suffix = vec![0, len1];
    let mut prev_len = 0;
    let mut i = 1;
    //the loop calculates longest_prefix_suffix[i] for i = 1 to n-1
    while i < len1 {
        if string1[i] == string2[prev_len] {
            longest_prefix_suffix[i] = prev_len + 1;
            prev_len+=1;
            i += 1;
        } else if prev_len == 0 {
            longest_prefix_suffix[i] = 0;
            i += 1;
        } else {
            prev_len = longest_prefix_suffix[prev_len - 1];
        }
    }

    i = 0;

    let mut k = longest_prefix_suffix[len1 - 1];
    // Match from that rotating point
    while k < len2 {
        if string2[k] != string1[i] {
            return false;
        }
        i += 1;
        k += 1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::rotation::is_rotation;

    #[test]
    fn rotation() {
        assert_eq!(is_rotation("abcd", "bcda"), true);
    }

    #[test]
    fn not_roration() {
        assert_eq!(is_rotation("abcd", "bacd"), false);
    }
}