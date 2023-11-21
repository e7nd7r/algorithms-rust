pub struct Codec {
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Self {
        Codec {}
    }
	
    pub fn encode(&self, _strs: Vec<String>) -> String {
        unimplemented!()
    }
    
	
    pub fn decode(&self, _s: String) -> Vec<String> {
        unimplemented!()
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
    use crate::array_hashing::encode_decode_v1::Codec;

    #[ignore]
    #[test]
    fn encode_works() {
        let codec = Codec::new();

        let dummy_input = vec!["Hello".to_owned(), "World".to_owned()];
        let encode_result = codec.encode(dummy_input);

        assert_eq!(encode_result, "[\"Hello\",\"World\"]".to_owned());
    }

    #[ignore]
    #[test]
    fn decode_works() {
        let codec = Codec::new();
        let s = codec.encode(vec!["hello".to_owned(), "world".to_owned()]);
        let expected = vec!["hello", "world"];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[ignore]
    #[test]
    fn decode_works_2() {
        let codec = Codec::new();
        let s = codec.encode(vec!["hello\\\\".to_owned(), "world".to_owned()]);
        let expected = vec!["hello\\", "world"];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[ignore]
    #[test]
    fn decode_works_3() {
        let codec = Codec::new();

        let s = codec.encode(vec!["\\".to_string()]);
        let expected = vec!["\\"];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[ignore]
    #[test]
    fn decode_works_4() {
        let codec = Codec::new();
        let s = codec.encode(vec![",".to_string()]);
        let expected = vec![","];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[ignore]
    #[test]
    fn decode_works_6() {
        let codec = Codec::new();
        let s = codec.encode(vec!["".to_string()]);
        let expected = vec![""];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }

    #[ignore]
    #[test]
    fn decode_works_7() {
        let codec = Codec::new();
        let s = codec.encode(vec!["\"".to_string()]);
        let expected = vec!["\""];
        let result = codec.decode(s.to_string());
        assert_eq!(result, expected);
    }
}
