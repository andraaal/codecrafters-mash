pub(crate) struct Args<'a> {
    raw: &'a str,
    pos: usize,
}

impl<'a> Iterator for Args<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.raw.as_bytes();
        let len = bytes.len();

        let mut start = None;
        let mut single_quotes = false;
        let mut segments = Vec::new();

        while self.pos < len {
            let b = bytes[self.pos];

            match b {
                b'\'' => {
                    if start.is_none() {
                        // token starts after opening quote
                        start = Some(self.pos + 1);
                        single_quotes = true;
                    } else if single_quotes {
                        // closing quote
                        let token = &self.raw[start.unwrap()..self.pos];
                        segments.push(token.to_string());
                        start = None;
                        self.pos += 1;
                        // skip if quote is empty
                        if token.is_empty() {
                            continue;
                        } else if self.pos < len && bytes[self.pos] == b'\'' {
                            self.pos += 1;
                            continue;
                        } else {
                            return Some(segments.join(""));
                        }
                    } else {
                        // We need to end the segment before the quotes
                        let token = &self.raw[start.unwrap()..self.pos];
                        segments.push(token.to_string());
                        start = None;
                    }
                }

                b if b.is_ascii_whitespace() && !single_quotes => {
                    if let Some(start) = start {
                        let token = &self.raw[start..self.pos];
                        self.pos += 1;
                        segments.push(token.to_string());
                        return Some(segments.join(""));
                    }
                }

                _ => {
                    if start.is_none() {
                        start = Some(self.pos);
                    }
                }
            }

            self.pos += 1;
        }

        start.map(|start| {
            segments.push(self.raw[start..len].to_string());
            segments.join("")
        })
    }
}

impl<'a> Args<'a> {
    pub(crate) fn new(raw: &'a str) -> Self {
        Self { raw, pos: 0 }
    }
}
