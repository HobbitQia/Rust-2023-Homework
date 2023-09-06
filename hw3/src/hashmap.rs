/*  hashmap.rs
    实现一个哈希宏，可以帮助我们初始化一个 HashMap。
*/
use std::collections::HashMap;

/*  hash_map
    哈希宏，初始化时被调用。
*/
macro_rules! hash_map {
    ( $($key: expr => $v: expr), *) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $v);
            )*
            map
        }
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_map() {
        let map = hash_map! {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6
        };
        assert_eq!(map["one"], 1);
        assert_eq!(map["two"], 2);
        assert_eq!(map["three"], 3);
        assert_eq!(map["four"], 4);
        assert_eq!(map["five"], 5);
        assert_eq!(map["six"], 6);
    }
}