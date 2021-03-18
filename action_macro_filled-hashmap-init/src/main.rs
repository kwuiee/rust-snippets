macro_rules! hashmap {
    ($($key: expr => $val: expr),*) => ({
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val);)*
        map
    })
}

fn main() {
    let counts = hashmap!["A" => 0, "C" => 1, "E" => 2];
    println!("{:?}", counts);
}
