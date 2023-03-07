use nom::{IResult, character::complete::satisfy, combinator::recognize, bytes::complete::take_while};
use nom::sequence::pair;
use crate::track::Track;

// This is what we want in the end - a function that parses a tja file
// and returns a Track struct containing everything we need.
fn parse_tja(s: &str) -> IResult<&str, Track> {
    todo!()
}

fn is_uppercase(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

// Parses a metadata tag, which will be an identifier that consists only of
// uppercase letters and numbers. It must start with an uppercase letter.
fn meta_tag(i: &str) -> IResult<&str, &str> {
    recognize(pair(satisfy(is_uppercase), take_while(|c| is_uppercase(c) || is_digit(c))))(i)
}

mod test {
    use super::meta_tag;

    #[test]
    fn test_meta_tag() {
        assert_eq!(meta_tag("TITLE:さいたま2000"), Ok((":さいたま2000", "TITLE")));
        assert_eq!(meta_tag("EXAM1:something"), Ok((":something", "EXAM1")));
    }
}
