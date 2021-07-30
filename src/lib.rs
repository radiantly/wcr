pub struct Count {
    pub chars: u64,
    pub lines: u64,
    pub words: u64,
}

impl Count {
    fn new() -> Self {
        Count {
            chars: 0,
            lines: 0,
            words: 0,
        }
    }
}

pub fn parse(buf: &Vec<u8>) -> Count {
    let mut count = Count::new();
    let mut last_char_was_space = true;
    for c in buf {
        if !last_char_was_space && *c == ' ' as u8 {
            count.words += 1;
            last_char_was_space = true;
        } else if *c == '\n' as u8 {
            if !last_char_was_space {
                count.words += 1;
                last_char_was_space = true;
            }
            count.lines += 1;
        } else {
            last_char_was_space = false;
        }
        count.chars += 1
    }

    if !last_char_was_space {
        count.words += 1;
    }
    count
}
