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
        let mut quotes = None;
        let mut segments = Vec::new();

        while self.pos < len {
            let b = bytes[self.pos];

            match b {
                b'\'' | b'"' => {
                    if quotes.is_none() {
                        // remember token type
                        quotes = Some(b);
                        let token = &self.raw[start.unwrap_or(self.pos)..self.pos];
                        segments.push(token.to_string());
                        start = Some(bytes.len().min(self.pos + 1));
                    } else if quotes == Some(b) {
                        // closing quote
                        let token = &self.raw[start.unwrap()..self.pos];
                        segments.push(token.to_string());
                        start = None;
                        quotes = None;
                        // skip if quote is empty
                        if token.is_empty() {
                            // Do nothing
                        } else if self.pos < len && Some(bytes[self.pos]) == quotes {
                            self.pos += 1;
                        } else {
                            self.pos += 1;
                            return Some(segments.join(""));
                        }
                    }
                }

                b if b.is_ascii_whitespace() && quotes.is_none() => {
                    if let Some(start) = start {
                        let token = &self.raw[start..self.pos];
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

        if let Some(start) = start {
            segments.push(self.raw[start..len].to_string());
        };
        let res = segments.join("");
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}

impl<'a> Args<'a> {
    pub(crate) fn new(raw: &'a str) -> Self {
        Self { raw, pos: 0 }
    }
}
