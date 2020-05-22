extern crate unic_ucd_name;

use std::io::{Write, Stdin};
use unic_ucd_name::Name;

type Result<T> = std::result::Result<T, std::io::Error>;

/// What type of columns there are
#[derive(Debug)]
enum Column {
    
    /// Index of this character from the start (0)
    CharacterIndex,

    /// How many bytes in this character is (start = 0)
    ByteIndex,

    Utf32,

    Utf8Bytes,

    /// The Glyph
    Glyph,

    /// Unicode name 
    Name
}

impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Turns stdin into an iterator of chars
struct StdinChars {
    stdin: Stdin,
    buf: Vec<char>,
    finished: bool,
}

impl StdinChars {
    fn new() -> Self {
        StdinChars{ stdin: std::io::stdin(), buf: vec![], finished: false }
    }
}

impl Iterator for StdinChars {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if self.buf.is_empty() {
            let mut next_line = String::new();
            let num_written = self.stdin.read_line(&mut next_line).unwrap();
            if num_written == 0 {
                self.finished = true;
                return None;
            }

            self.buf.extend(next_line.chars());
            self.buf.reverse();
        }

        self.buf.pop()
    }
}

fn main() -> Result<()> {
    let cols = &[
        Column::CharacterIndex,
        Column::ByteIndex,
        Column::Utf32,
        Column::Utf8Bytes,
        Column::Glyph,
        Column::Name,
    ];

    let mut stdin = StdinChars::new();
    let mut stdout = std::io::stdout();

    format_output(cols, &mut stdin, &mut stdout)?;

    Ok(())
}

fn format_column(col: &Column, text: &str, output: &mut impl Write) -> Result<()> {
    match col {
        Column::CharacterIndex => write!(output, "{:>9}", text),
        Column::ByteIndex => write!(output, "{:>5}", text),
        Column::Utf8Bytes => write!(output, "{:<12}", text),
        Column::Glyph => write!(output, "{:^8}", text),

        _ => write!(output, "{}", text),

    }
}

fn format_output(columns: &[Column], input: &mut impl Iterator<Item=char>, output: &mut impl Write) -> Result<()> {
    let mut first;
    for line in lines_for_input(columns, input) {
        first = true;
        for (text, col) in line.iter().zip(columns) {
            if !first {
                write!(output, "  ")?;
            }
            format_column(col, text, output)?;
            first = false;
        }
        write!(output, "\n")?;
    }
    Ok(())
}


fn lines_for_input<'a>(columns: &'a [Column], input: &'a mut impl Iterator<Item=char>) -> impl Iterator<Item=Vec<String>> + 'a {
    let mut byte_index = 0;

    input.enumerate().flat_map(move |(char_index, c)| {
        let cols = columns_for_char(c, columns, char_index, byte_index);
        byte_index += c.len_utf8();
        if char_index % 100 == 0 {
            vec![headers(columns), cols]
        } else {
            vec![cols]
        }
    })
}

fn headers(columns: &[Column]) -> Vec<String> {
    columns.iter().map(|c|
        match c {
            Column::CharacterIndex => "character",
            Column::ByteIndex => "byte",
            Column::Utf32 => "UTF-32",
            Column::Utf8Bytes => "encoded as",
            Column::Glyph => "glyph",
            Column::Name => "name",
        }
    ).map(|s| s.to_string()).collect()
}

fn unicode_name(c: char) -> String {
    if (c as u64) < 0x20 {
        // the crate doesn't have control characters
        let s = match c as u64 {
            0x00 => "NULL",
            0x01 => "START OF HEADING",
            0x02 => "START OF TEXT",
            0x03 => "END OF TEXT",
            0x04 => "END OF TRANSMISSION",
            0x05 => "ENQUIRY",
            0x06 => "ACKNOWLEDGE",
            0x07 => "BELL",
            0x08 => "BACKSPACE",
            0x09 => "CHARACTER TABULATION",
            0x0A => "LINE FEED (LF)",
            0x0B => "LINE TABULATION",
            0x0C => "FORM FEED (FF)",
            0x0D => "CARRIAGE RETURN (CR)",
            0x0E => "SHIFT OUT",
            0x0F => "SHIFT IN",
            0x10 => "DATA LINK ESCAPE",
            0x11 => "DEVICE CONTROL ONE",
            0x12 => "DEVICE CONTROL TWO",
            0x13 => "DEVICE CONTROL THREE",
            0x14 => "DEVICE CONTROL FOUR",
            0x15 => "NEGATIVE ACKNOWLEDGE",
            0x16 => "SYNCHRONOUS IDLE",
            0x17 => "END OF TRANSMISSION BLOCK",
            0x18 => "CANCEL",
            0x19 => "END OF MEDIUM",
            0x1A => "SUBSTITUTE",
            0x1B => "ESCAPE",
            0x1C => "INFORMATION SEPARATOR FOUR",
            0x1D => "INFORMATION SEPARATOR THREE",
            0x1E => "INFORMATION SEPARATOR TWO",
            0x1F => "INFORMATION SEPARATOR ONE",
            _ => unreachable!(),
        };
        s.to_string()
    } else {
        Name::of(c).map(|n| n.to_string()).unwrap_or_else(|| "NAME UNKNOWN".to_string())
    }
}

fn columns_for_char(c: char, columns: &[Column], char_idx: usize, byte_idx: usize) -> Vec<String> {
    let mut utf8_bytes = vec![0; 6];
    c.encode_utf8(&mut utf8_bytes);
    columns.iter().map(|col| match col {
        Column::CharacterIndex => format!("{}", char_idx),
        Column::ByteIndex => format!("{}", byte_idx),
        Column::Utf32 => format!("{:<06X}", c as u32),
        Column::Utf8Bytes => {
            let mut utf8_bytes = vec![0; 6];
            c.encode_utf8(&mut utf8_bytes);
            format!("{}", utf8_bytes.iter().take(c.len_utf8()).map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" "))
        },
        Column::Glyph => format!("{}", c.escape_debug()),
        Column::Name => unicode_name(c),
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_lines(s: &str, columns: &[Column]) -> Vec<Vec<String>> {
        lines_for_input(columns, &mut s.chars()).collect()
    }

    #[test]
    fn test_empty() {
        assert_eq!(str_to_lines("", &[]), vec![] as Vec<Vec<String>>);
        assert_eq!(str_to_lines("", &[Column::Name]), vec![] as Vec<Vec<String>>);
    }

    #[test]
    fn test_single1() {
        assert_eq!(columns_for_char('a', &[Column::Utf32, Column::Utf8Bytes, Column::Glyph, Column::Name], 0, 0),
            vec![
                "000061", "61", "a",
                "LATIN SMALL LETTER A"
            ]);

        assert_eq!(columns_for_char('ä', &[Column::Utf32, Column::Utf8Bytes, Column::Glyph, Column::Name], 0, 0),
            vec![
                "0000E4", "C3 A4", "ä",
                "LATIN SMALL LETTER A WITH DIAERESIS"
            ]);

        assert_eq!(columns_for_char('→', &[Column::Utf32, Column::Utf8Bytes, Column::Glyph, Column::Name], 0, 0),
            vec![
                "002192", "E2 86 92", "→",
                "RIGHTWARDS ARROW"
            ]);

        assert_eq!(columns_for_char('\n', &[Column::Utf32, Column::Utf8Bytes, Column::Name], 0, 0),
            vec![
                "00000A", "0A", "LINE FEED (LF)"
            ]);

    }

    #[test]
    fn test_many1() {
        assert_eq!(str_to_lines("a", &[Column::CharacterIndex, Column::ByteIndex, Column::Utf32, Column::Utf8Bytes, Column::Glyph, Column::Name]),
            vec![
                vec!["character", "byte", "UTF-32", "encoded as", "glyph", "name"],
                vec!["0", "0", "000061", "61", "a", "LATIN SMALL LETTER A"],
            ]);

        assert_eq!(str_to_lines("abc", &[Column::CharacterIndex, Column::ByteIndex, Column::Utf32, Column::Utf8Bytes, Column::Glyph, Column::Name]),
            vec![
                vec!["character", "byte", "UTF-32", "encoded as", "glyph", "name"],
                vec!["0", "0", "000061", "61", "a", "LATIN SMALL LETTER A"],
                vec!["1", "1", "000062", "62", "b", "LATIN SMALL LETTER B"],
                vec!["2", "2", "000063", "63", "c", "LATIN SMALL LETTER C"],
            ]);

        assert_eq!(str_to_lines("🙂🙂", &[Column::CharacterIndex, Column::ByteIndex, Column::Utf32, Column::Utf8Bytes, Column::Glyph, Column::Name]),
            vec![
                vec!["character", "byte", "UTF-32", "encoded as", "glyph", "name"],
                vec!["0", "0", "01F642", "F0 9F 99 82", "🙂", "SLIGHTLY SMILING FACE"],
                vec!["1", "4", "01F642", "F0 9F 99 82", "🙂", "SLIGHTLY SMILING FACE"],
            ]);

        assert_eq!(str_to_lines("🏳️‍🌈\n", &[Column::CharacterIndex, Column::ByteIndex, Column::Utf32, Column::Utf8Bytes, Column::Glyph, Column::Name]),


            vec![
                vec!["character", "byte", "UTF-32", "encoded as", "glyph", "name"],
                vec!["0", "0", "01F3F3", "F0 9F 8F B3", "🏳", "WAVING WHITE FLAG"],
                vec!["1", "4", "00FE0F", "EF B8 8F", "\\u{fe0f}", "VARIATION SELECTOR-16"],
                vec!["2", "7", "00200D", "E2 80 8D", "\\u{200d}", "ZERO WIDTH JOINER"],
                vec!["3", "10", "01F308", "F0 9F 8C 88", "🌈", "RAINBOW"],
                vec!["4", "14", "00000A", "0A", "\\n", "LINE FEED (LF)"],
            ]);

    }

    #[test]
    fn test_writing1() {
        let input = "a";
        let mut output = Vec::new();

        format_output(&[Column::CharacterIndex, Column::Name], &mut input.chars(), &mut output).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "character  name\n        0  LATIN SMALL LETTER A\n"
        );
    }


}
