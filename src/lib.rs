use std::ops::AddAssign;
#[derive(Debug, PartialEq)]
pub struct Count {
    pub chars: usize,
    pub lines: usize,
    pub words: usize,
}

impl Count {
    pub fn new() -> Self {
        Count {
            chars: 0,
            lines: 0,
            words: 0,
        }
    }
}

impl AddAssign for Count {
    fn add_assign(&mut self, rhs: Self) {
        self.chars += rhs.chars;
        self.lines += rhs.lines;
        self.words += rhs.words;
    }
}

pub fn parse(buf: Vec<u8>) -> Count {
    let mut count = Count::new();
    count.chars = buf.len();
    let mut last_char_was_space = true;
    for c in buf {
        if c == ' ' as u8 {
            if last_char_was_space {
                continue;
            }
            count.words += 1;
            last_char_was_space = true;
        } else if c == '\n' as u8 {
            if !last_char_was_space {
                count.words += 1;
                last_char_was_space = true;
            }
            count.lines += 1;
        } else {
            last_char_was_space = false;
        }
    }

    if !last_char_was_space {
        count.words += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_count(chars: usize, lines: usize, words: usize) -> Count {
        Count {
            chars,
            lines,
            words,
        }
    }

    #[test]
    fn no_words() {
        assert_eq!(parse(vec![]), create_count(0, 0, 0));
    }

    #[test]
    fn one_word() {
        assert_eq!(parse("hello".as_bytes().into()), create_count(5, 1, 1));
    }

    #[test]
    fn basic_phrase() {
        assert_eq!(
            parse(
                "Hi you seem to have an air oh mystery surrounding you"
                    .as_bytes()
                    .into()
            ),
            create_count(53, 1, 11)
        );
    }

    #[test]
    fn bad_space() {
        assert_eq!(
            parse(
                "  There  seems  to be a weird number   of spaces    here. Super unusual, right?   "
                    .as_bytes()
                    .into()
            ),
            create_count(82, 1, 13)
        )
    }
}
