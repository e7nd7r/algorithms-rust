use std::fmt;

pub struct Codec {
}

#[derive(PartialEq, Clone, Debug)]
pub enum State {
    Start,
    SquareBracketOpened,
    OpenedQuote,
    Loading,
    Escaped,
    ClosedQuote,
    CommaSeparated,
    SquareBracketClosed,
    Err(String),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Start => write!(f, "Start"),
            State::SquareBracketOpened => write!(f, "SquareBracketOpened"),
            State::OpenedQuote => write!(f, "OpenedQuote"),
            State::Loading => write!(f, "Loading"),
            State::Escaped => write!(f, "Escaped"),
            State::ClosedQuote => write!(f, "ClosedQuote"),
            State::CommaSeparated => write!(f, "CommaSeparated"),
            State::SquareBracketClosed => write!(f, "SquareBracketClosed"),
            State::Err(err) => write!(f, "{}", err),
        }
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }
	
    fn encode(&self, strs: Vec<String>) -> String {

        let encoded_elements: Vec<String> = strs
            .iter()
            .map(|str| {
                let escaped = str
                    .replace("\\", "\\\\")
                    .replace("\"", "\\\"");
                format!("\"{}\"", escaped)
            })
            .collect();

        return format!("[{}]", encoded_elements.join(","))
    }
    
	
    fn decode(&self, s: String) -> Vec<String> {
        let mut state = State::Start;
        let mut result = Vec::<String>::new();
        let mut chars = s.chars();
        let mut current = chars.next();

        loop {
            state = match state {
                State::Start => {
                    let next_state = match current {
                        Some('[') => State::SquareBracketOpened,
                        Some(any) => State::Err(format!("Invalid character {any}. Expected [.").to_owned()),
                        _ => State::Err(format!("State {}. Unexpected error.", state).to_owned()),
                    };

                    current = chars.next();

                    next_state
                },
                State::SquareBracketOpened => {
                    let next_state = match current {
                        Some('"') => State::OpenedQuote,
                        Some(any) => State::Err(format!("Invalid character {any}. Expected \".").to_owned()),
                        None => State::Err(format!("State {}. Unexpected error.", state).to_owned()),
                    };

                    current = chars.next();

                    next_state
                },
                State::OpenedQuote => {
                    result.push(String::new());
                    State::Loading
                },
                State::Loading => {
                    let str_buffer = result.last_mut().unwrap();

                    let next_state = match current {
                        Some('"') => State::ClosedQuote,
                        Some('\\') => State::Escaped,
                        Some(any) => {
                            str_buffer.push(any);
                            State::Loading
                        },
                        _ => State::Err(format!("State {}. Unexpected error.", state).to_owned()),
                    };

                    current = chars.next();

                    next_state
                },
                State::Escaped => {
                    let next_state = match current {
                        Some('\\') | Some('\"') => { 
                            result.last_mut().unwrap().push(current.unwrap());
                            State::Loading 
                        },
                        Some(any) => State::Err(format!("State: {}, Invalid character {}. Expected \" or \\.", state, any).to_owned()),
                        _ => State::Err(format!("State {}. Unexpected error.", state).to_owned()),
                    };

                    current = chars.next();

                    next_state
                }
                State::ClosedQuote => {
                    let next_state = match current {
                        Some(',') => State::CommaSeparated,
                        Some(']') => State::SquareBracketClosed,
                        Some(any) => State::Err(format!("State: {}, Invalid character {}. Expected , or ].", state, any).to_owned()),
                        _ => State::Err(format!("State: {}. Unexpected error.", state).to_owned()),
                    };

                    current = chars.next();

                    next_state
                },
                State::CommaSeparated => {
                    let next_state = match current {
                        Some('"') => State::OpenedQuote,
                        Some(any) => State::Err(format!("State: {}, Invalid character {}. Expected \".", state, any).to_owned()),
                        _ => State::Err(format!("State: {}. Unexpected error.", state).to_owned()),
                    };

                    current = chars.next();

                    next_state
                },
                State::SquareBracketClosed => {
                    let next_state = match current {
                        None => return result,
                        Some(any) => State::Err(format!("State {}, Invalid character {}. Expected end of string.", state, any).to_owned()),
                    };

                    current = chars.next();

                    next_state
                },
                State::Err(err) => {
                    panic!("{}", err);
                }
            }
        };
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: Vec<String> = obj.decode(s);
 */

#[cfg(test)]
mod tests {
    use crate::array_hashing::encode_decode::Codec;

    #[test]
    fn encode_works() {
        let codec = Codec::new();

        let dummy_input = vec!["Hello".to_owned(), "World".to_owned()];
        let encode_result = codec.encode(dummy_input);

        assert_eq!(encode_result, "[\"Hello\",\"World\"]".to_owned());
    }

    #[test]
    fn decode_works() {
        let codec = Codec::new();
        let s = codec.encode(vec!["hello".to_owned(), "world".to_owned()]);
        let expected = vec!["hello", "world"];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn decode_works_2() {
        let codec = Codec::new();
        let s = codec.encode(vec!["hello\\\\".to_owned(), "world".to_owned()]);
        let expected = vec!["hello\\", "world"];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn decode_works_3() {
        let codec = Codec::new();

        let s = codec.encode(vec!["\\".to_string()]);
        let expected = vec!["\\"];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn decode_works_4() {
        let codec = Codec::new();
        let s = codec.encode(vec![",".to_string()]);
        let expected = vec![","];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn decode_works_6() {
        let codec = Codec::new();
        let s = codec.encode(vec!["".to_string()]);
        let expected = vec![""];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn decode_works_7() {
        let codec = Codec::new();
        let s = codec.encode(vec!["\"".to_string()]);
        let expected = vec!["\""];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }
}
