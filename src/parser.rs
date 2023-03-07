use crate::track::{Track, TrackMetadata};
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{char, line_ending, multispace0, space0};
use nom::error::ParseError;
use nom::multi::{many0_count, many1, many_m_n};
use nom::sequence::{pair, separated_pair, terminated};
use nom::{character::complete::satisfy, combinator::recognize, IResult};

fn uppercase(i: &str) -> IResult<&str, char> {
    satisfy(|c| ('A'..='Z').contains(&c))(i)
}

fn digit(i: &str) -> IResult<&str, char> {
    satisfy(|c| ('0'..='9').contains(&c))(i)
}

// RUST MOMENT
fn line_of<'a, F, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    terminated(inner, pair(many_m_n(0, 1, line_ending), multispace0))
}

// Parses a metadata tag, which will be an identifier that consists only of
// uppercase letters and numbers. It must start with an uppercase letter.
fn metadata_tagname(i: &str) -> IResult<&str, &str> {
    recognize(pair(uppercase, many0_count(alt((uppercase, digit)))))(i)
}

fn metadata_pair(i: &str) -> IResult<&str, (&str, &str)> {
    line_of(separated_pair(
        metadata_tagname,
        pair(char(':'), space0),
        is_not("\n\r"),
    ))(i)
}

fn metadata_list(i: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(metadata_pair)(i)
}

// This is what we want in the end - a function that parses a tja file
// and returns a Track struct containing everything we need.

/// Parses a TJA file and returns a Track struct containing all the
/// info about the song. See `Track` for details
pub fn parse_tja(s: &str) -> IResult<&str, Track> {
    todo!()
}

mod test {
    use super::{metadata_list, metadata_pair, metadata_tagname};

    #[test]
    fn test_meta_tag() {
        assert_eq!(
            metadata_tagname("TITLE:さいたま2000"),
            Ok((":さいたま2000", "TITLE"))
        );
        assert_eq!(
            metadata_tagname("EXAM1:something"),
            Ok((":something", "EXAM1"))
        );
    }

    #[test]
    fn test_meta_pair() {
        assert_eq!(
            metadata_pair("TITLE:さいたま2000\n"),
            Ok(("", ("TITLE", "さいたま2000")))
        );
        assert_eq!(
            metadata_pair("TITLE:さいたま2000\r\n"),
            Ok(("", ("TITLE", "さいたま2000")))
        );
        assert_eq!(
            metadata_pair("EXAM1:something"),
            Ok(("", ("EXAM1", "something")))
        );
    }

    #[test]
    fn test_meta_list() {
        let test_header = "TITLE:POP TEAM EPIC
SUBTITLEJA:TVアニメ「ポプテピピック」OPテーマ
BPM:142

#START";

        assert_eq!(
            metadata_list(test_header),
            Ok((
                "#START",
                vec![
                    ("TITLE", "POP TEAM EPIC"),
                    ("SUBTITLEJA", "TVアニメ「ポプテピピック」OPテーマ"),
                    ("BPM", "142")
                ]
            ))
        );
    }
}
