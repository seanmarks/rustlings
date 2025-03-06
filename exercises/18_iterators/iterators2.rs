// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            // Initial
            // chars.fold(first.to_string().to_uppercase(), |mut s, c| { s.push(c); s })
            // Better
            first.to_uppercase().to_string() + chars.as_str()
        },
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| capitalize_first(s)).collect()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // --- Init ---
    // let edited_words = capitalize_words_vector(words);
    // let mut it = edited_words.iter();
    // match it.next() {
    //     None => String::new(),
    //     Some(first_word) => {
    //         it.fold(first_word.clone(), |mut s, word| {s.push_str(word); s} )
    //     },
    // }
    // --- Better ---
    // .collect() -> String automatically joins the output collection
    words.iter().map(|s| capitalize_first(s)).collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
