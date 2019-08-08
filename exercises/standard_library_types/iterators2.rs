// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer
// Step 1. Complete the `capitalize_first` function to pass the first two cases
// Step 2. Apply the `capitalize_first` function to a vector of strings, ensuring that it
// Step 3. Apply the `capitalize_first` function again to a list, but try and ensure it returns a single string
// As always, there are hints below!

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let mut owned_string: String = "".to_owned();
            owned_string.push(first.to_ascii_uppercase());
            owned_string.push_str(c.as_str());
            return owned_string;
        }
    }
}

pub fn capitalized_words(input: Vec<&str>) -> Vec<String> {
    let mut owned_strings: Vec<String> = Vec::new();
    for s in input {
        owned_strings.push(capitalize_first(s));
    }
    return owned_strings;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = capitalized_words(words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = capitalized_words(words);
        assert_eq!(capitalized_words.join(""), "Hello World");
    }
}
































// Step 1
// You need to call something on `first` before it can be collected
// Currently it's type is `char`. Have a look at the methods that are available on that type:
// https://doc.rust-lang.org/std/primitive.char.html


























// Step 2
// First you'll need to turn the Vec into an iterator
// Then you'll need to apply your function unto each item in the vector
// P.s. Don't forget to collect() at the end!





























// Step 3.
// This is very similar to the previous test. The only real change is that you will need to
// alter the type that collect is coerced into. For a bonus you could try doing this with a
// turbofish
