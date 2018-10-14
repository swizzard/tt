
pub mod tree {

    pub struct Node {
        token: String,
        count: u64,
        children: Vec<Box<Node>>,
    }

    impl Node {
        fn new(s: String) -> Node {
            Node {
                token: s,
                count: 0,
                children: Vec::new(),
            }
        }
    }
}

pub mod tokenizer {
    use std::str::Chars;
    use std::fs::read_to_string;

    pub struct WSP<'a> {
        text: Chars<'a>,
        curr: String,
        punc: Option<String>,
        wb: bool,
    }

    impl<'a> WSP<'a> {
        pub fn new(s: &'a str) -> Self {
            WSP {
                text: s.chars(),
                curr: String::new(),
                punc: None,
                wb: false,
            }
        }

        pub fn from_file(pth: &str) -> Self {
            let f = read_to_string(pth).unwrap();
            WSP::new(&f)
        }

    }

    impl<'a> Iterator for WSP<'a> {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            match self.punc.take() {
                Some(p) => {
                    Some(p)
                },
                None => {
                    loop {
                        match self.text.next() {
                            None => {
                                return None
                            },
                            Some(c) => {
                                if self.wb {
                                    self.wb = false;
                                    self.curr = c.to_string();
                                } else if c.is_whitespace() {
                                    self.wb = true;
                                    self.punc = None;
                                    return Some(self.curr.clone())
                                } else if c.is_ascii_punctuation() {
                                    self.wb = true;
                                    self.punc = Some(c.to_string());
                                    return Some(self.curr.clone())
                                } else {
                                    self.curr.push(c);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
