use std::collections::HashMap;

struct TimeMap {
    inner:  HashMap<String, Vec<(i32, String)>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

 #[allow(dead_code)]
impl TimeMap {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if !self.inner.contains_key(&key) {
            self.inner.insert(key, vec![(timestamp, value)]);

            return;
        }

        let elements = self.inner.get_mut(&key).unwrap();

        elements.push((timestamp, value))
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if !self.inner.contains_key(&key) {
            return "".to_owned();
        }

        let elements = self.inner.get(&key).unwrap();

        let index = self.find(key, timestamp);

        if index == -1 {
            return "".to_owned();
        }

        let (_, value) = &elements[index as usize];

        return value.clone();
    }

    pub(crate) fn find(&self, key: String, timestamp: i32) -> i32 {
        let elements = self.inner.get(&key).unwrap();

        let mut left = 0i32;
        let mut right = (elements.len()  - 1) as i32;

        while left <= right {
            let mid = (left + right) / 2 + ((left + right) % 2 != 0) as i32;
            let (mid_ts, _) = elements[mid as usize];

            if timestamp == mid_ts {
                return mid;
            }

            if timestamp < mid_ts {
                right = mid - 1 ;
            } else {
                left = mid + 1;
            }
        }

        right
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

 #[cfg(test)]
mod test_super {
    use crate::binary_search::time_based_key_value::TimeMap;
 
    #[test]
    fn test_timemap() {
        let mut timemap = TimeMap::new();

        timemap.set("foo".to_owned(), "bar".to_owned(), 1);
        assert_eq!(timemap.get("foo".to_owned(), 1), "bar");
        assert_eq!(timemap.get("foo".to_owned(), 3), "bar");
        
        timemap.set("foo".to_owned(), "bar2".to_owned(), 4);
        assert_eq!(timemap.get("foo".to_owned(), 4), "bar2");
        assert_eq!(timemap.get("foo".to_owned(), 5), "bar2");
    }

    #[test]
    fn test_timemap01() {
        let mut timemap = TimeMap::new();

        timemap.set("foo".to_owned(), "foo".to_owned(), 1);
        timemap.set("foo".to_owned(), "bar".to_owned(), 5);
        timemap.set("foo".to_owned(), "baz".to_owned(), 8);
        assert_eq!(timemap.get("foo".to_owned(), 2), "foo");
        assert_eq!(timemap.get("foo".to_owned(), 5), "bar");
        assert_eq!(timemap.get("foo".to_owned(), 9), "baz");
        
        timemap.set("foo".to_owned(), "bar2".to_owned(), 4);
        assert_eq!(timemap.get("foo".to_owned(), 4), "foo");
        assert_eq!(timemap.get("foo".to_owned(), 5), "bar");
    }

    #[test]
    fn test_timemap3() {
        let mut timemap = TimeMap::new();

        timemap.set("love".to_owned(), "high".to_owned(), 10);
        timemap.set("love".to_owned(), "low".to_owned(), 20);
        assert_eq!(timemap.get("love".to_owned(), 5), "");
        assert_eq!(timemap.get("love".to_owned(), 15), "high");
        assert_eq!(timemap.get("love".to_owned(), 20), "low");
        assert_eq!(timemap.get("love".to_owned(), 25), "low");
    }

    /*
        ["TimeMap","set","set","get","get","get","get","get"]
        [[],["love","high",10],["love","low",20],["love",5],["love",10],["love",15],["love",20],["love",25]]
    */
}
